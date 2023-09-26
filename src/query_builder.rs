use mysql_common::{prelude::ToValue, Value};
use sql_paginatorr::LimitOffsetPair;

use crate::{
    params::Params, sql::SqlOrder, wp_post::post_status::PostStatus, PostType, SqlSearchOperators,
};

type StmtValues = Vec<Value>;

pub struct QueryBuilder<'a> {
    params: Params<'a>,
    query: String,
    values: StmtValues,
}

pub struct QueryAndValues(pub String, pub Vec<Value>);

impl<'a> QueryBuilder<'a> {
    pub fn new(params: Params<'a>) -> Self {
        Self {
            params,
            query: String::new(),
            values: vec![],
        }
    }

    pub fn query(mut self) -> QueryAndValues {
        let params = self.params;

        macro_rules! add_if_some_id {
            ($prop: ident, $query: expr) => {
                if let Some(id) = params.$prop {
                    self.query.push_str($query);
                    self.values.push(Value::UInt(id));
                }
            };
        }

        macro_rules! add_multiple_if_some_ids {
            ($prop: ident, $q: expr) => {
                if let Some(ids) = params.$prop {
                    let q_marks = implode_to_question_mark(&ids);
                    self.query.push_str(&format!($q, q_marks));

                    let ids = ids.into_iter().map(|id| Value::UInt(id));

                    self.values.extend(ids);
                }
            };
        }

        self.query.push_str(
            "SELECT DISTINCT(wp_posts.ID),post_author,comment_count,post_parent,menu_order,
            post_date,post_date_gmt,post_modified,post_modified_gmt,
            post_status,post_content,post_title,post_excerpt,comment_status,ping_status,
            post_password,post_name,to_ping,pinged,post_content_filtered,guid,
            post_type,post_mime_type
            FROM wp_posts",
        );

        let join_term = check_if_term_join_necessary(&params);
        let join_meta = check_if_meta_join_necessary(&params);
        let join_user = check_if_user_join_necessary(&params);
        let has_single_date = has_valid_single_date(&params);

        if join_meta {
            self.query
                .push_str(" INNER JOIN wp_postmeta ON wp_postmeta.post_id = wp_posts.ID");
        }

        if join_term {
            self.query.push_str(
                " INNER JOIN wp_term_relationships
            ON wp_posts.ID = wp_term_relationships.object_id
            INNER JOIN wp_term_taxonomy ON wp_term_taxonomy.term_taxonomy_id = wp_term_relationships.term_taxonomy_id
            INNER JOIN wp_terms ON wp_terms.term_id = wp_term_taxonomy.term_id",
            );
        }

        if join_user {
            self.query
                .push_str(" INNER JOIN wp_users ON wp_users.ID = wp_posts.post_author");
        }

        // Avoid dangling WHERE issue
        self.query.push_str(" WHERE 1 = 1");

        add_if_some_id!(author, " AND post_author = ?");

        if let Some(author_name) = params.author_name {
            self.query.push_str(" AND wp_users.user_nicename = ?");
            self.values.push(Value::Bytes(author_name.into()));
        }

        add_multiple_if_some_ids!(author__in, " AND post_author IN ({})");

        add_multiple_if_some_ids!(author__not_in, " AND post_author NOT IN ({})");

        /* Add Post Status conditions */
        if let Some(post_status) = params.post_status {
            push_post_status(&mut self.query, &mut self.values, &post_status);
        } else {
            self.query
                .push_str(&format!(" AND post_status = '{}'", PostStatus::Publish));
        }

        /* Add category, tag, and term conditions */
        if let Some(term_slugs) = params.term_slug_and {
            for term_slug in term_slugs.into_iter() {
                self.query.push_str(" AND wp_terms.slug = ?");
                self.values.push(Value::Bytes(term_slug.into()));
            }
        }

        if let Some(term_slugs) = params.term_slug_in {
            let q_marks = implode_to_question_mark(&term_slugs);

            self.query
                .push_str(&format!(" AND wp_terms.slug IN ({})", q_marks));

            let values = term_slugs.into_iter().map(|slug| Value::Bytes(slug.into()));

            self.values.extend(values);
        }

        add_multiple_if_some_ids!(term_in, " AND wp_terms.term_id IN ({})");

        add_multiple_if_some_ids!(term_not_in, " AND wp_terms.term_id NOT IN ({})");

        /* Tax Query */
        if let Some(queries) = params.tax_query {
            for (relationship, tax_queries) in queries.into_iter() {
                self.query.push_str(" AND");

                // Operator for comparing multiple tax queries
                let rel_str = relationship.to_string();

                for (i, tax_q) in tax_queries.into_iter().enumerate() {
                    // Add internal operator if length is greater than 1
                    if i > 0 {
                        self.query.push_str(" ");
                        self.query.push_str(&rel_str);
                    }

                    let q_marks = implode_to_question_mark(&tax_q.terms);

                    let prefix = "wp";
                    self.query.push_str(&format!(
                        " {}_{} {} ({})",
                        prefix, tax_q.field, tax_q.operator, q_marks
                    ));
                    self.values
                        .extend(tax_q.terms.into_iter().map(|v| Value::Bytes(v.into())))
                }
            }
        }

        /* Add search conditions */
        if let Some(keyword) = params.s {
            self.query
                .push_str(" AND wp_posts.post_content LIKE CONCAT('%',?,'%') OR wp_posts.post_title LIKE CONCAT('%',?,'%')");
            self.values.push(Value::Bytes(keyword.as_bytes().to_vec())); // Clone this so it can be used again
            self.values.push(Value::Bytes(keyword.into()));
        }

        /* Add page/post conditions */
        add_if_some_id!(p, " AND wp_posts.ID = ?");

        if let Some(name) = params.name {
            self.query.push_str(" AND wp_posts.post_name = ?");
            self.values.push(Value::Bytes(name.into()));
        }

        /* Post types */
        push_post_type(&mut self.query, &mut self.values, params.post_type);

        add_if_some_id!(post_parent, " AND wp_posts.post_parent = ?");

        add_multiple_if_some_ids!(post_parent__in, " AND wp_posts.post_parent IN ({})");

        add_multiple_if_some_ids!(post_parent__not_in, " AND wp_posts.post_parent NOT IN ({})");

        add_multiple_if_some_ids!(post__in, " AND wp_posts.ID IN ({})");

        add_multiple_if_some_ids!(post__not_in, " AND wp_posts.ID NOT IN ({})");

        if let Some(p_names) = params.post_name__in {
            let q_marks = implode_to_question_mark(&p_names);
            self.query
                .push_str(&format!(" AND wp_posts.post_name IN ({})", q_marks));

            let ids = p_names.into_iter().map(|name| Value::Bytes(name.into()));

            self.values.extend(ids);
        }

        /* Add specific date condition */
        if has_single_date {
            let date = Value::Date(
                params.year.unwrap_or(2023),
                params.monthnum.unwrap_or(1),
                params.day.unwrap_or(1),
                params.hour.unwrap_or(0),
                params.minute.unwrap_or(0),
                params.second.unwrap_or(0),
                0u32,
            );
            self.query.push_str(" AND wp_posts.post_date = ?");
            self.values.push(date);
        }

        /* Add date queries */
        if let Some(date_queries) = params.date_query {
            for dq in date_queries {
                let col = &dq.column;
                if dq.year.is_some() && dq.month.is_some() && dq.day.is_some() {
                    self.query.push_str(&format!(" AND wp_posts.{} = ?", col));
                    self.values.push(dq.to_value());
                }

                let op = dq.relation.to_string();
                if let Some(after) = dq.after {
                    let d_op = if dq.inclusive { ">=" } else { ">" };
                    self.query
                        .push_str(&format!(" {} wp_posts.{} {} ?", &op, col, d_op));
                    self.values.push(after.to_value());
                }

                if let Some(before) = dq.before {
                    let d_op = if dq.inclusive { "<=" } else { "<" };
                    self.query
                        .push_str(&format!(" {} wp_posts.{} {} ?", &op, col, d_op));
                    self.values.push(before.to_value());
                }
            }
        }

        /* Add meta conditions */
        let meta_op = params
            .meta_compare
            .as_ref()
            .unwrap_or(&crate::sql::SqlSearchOperators::Equals)
            .to_string();
        if let Some(meta_k) = params.meta_key {
            self.query
                .push_str(&format!(" AND wp_postmeta.meta_key {} ?", meta_op));
            self.values.push(Value::Bytes(meta_k.into()));
        }

        if let Some(meta_v) = params.meta_value {
            self.query
                .push_str(&format!(" AND wp_postmeta.meta_value {} ?", meta_op));
            self.values.push(Value::Bytes(meta_v.into_bytes()));
        }

        if let Some(meta_v) = params.meta_value_num {
            self.query
                .push_str(&format!(" AND wp_postmeta.meta_value {} ?", meta_op));
            self.values.push(Value::Int(meta_v));
        }

        if let Some(query_rel_map) = params.meta_query {
            for (relation, queries) in query_rel_map {
                self.query.push_str(" AND");
                for (i, query) in queries.into_iter().enumerate() {
                    // Print relation (OR/AND) if more than one query in the group, after first
                    if i > 0 {
                        self.query.push_str(" ");
                        self.query.push_str(relation.to_string().as_str());
                    }

                    // If Exists or NOT EXISTS, we need to make a different query structure
                    match query.compare {
                        op @ (SqlSearchOperators::Exists | SqlSearchOperators::NotExists) => {
                            self.query.push_str(&format!(
                                // Must have AND post_id to ensure that we are not comparing all meta rows for exists/not exist
                                " {} (SELECT * FROM wp_postmeta WHERE meta_key = ? AND wp_postmeta.post_id = wp_posts.ID)",
                                op.to_string()
                            ));
                            // Only push keyname if exists query
                            self.values.push(Value::Bytes(query.key.into_bytes()));
                        }
                        _ => {
                            let op = query.compare.to_string();
                            self.query.push_str(&format!(
                                " (wp_postmeta.meta_value {} ? AND wp_postmeta.meta_key {} ?)",
                                op, op
                            ));
                            self.values.push(Value::Bytes(query.value.into_bytes()));
                            self.values.push(Value::Bytes(query.key.into_bytes()));
                        }
                    };
                }
            }
        }

        /* Add order conditions */
        if let Some(orderby) = params.orderby {
            let order = params.order.unwrap_or(SqlOrder::Desc).clone().to_string();
            self.query
                .push_str(&format!(" ORDER BY {} {}", orderby.to_string(), order))
        }

        /* Add pagination */
        if let Some(page) = params.page {
            let LimitOffsetPair { offset, limit } = sql_paginatorr::for_page(
                page as usize,
                params.posts_per_page.unwrap_or(10) as usize,
            );

            self.query.push_str(" LIMIT ? OFFSET ?;");
            self.values.push(Value::UInt(limit as u64));
            self.values.push(Value::UInt(offset as u64));
        } else {
            let limit = params.posts_per_page.unwrap_or(10);
            self.query.push_str(" LIMIT ?;");
            self.values.push(Value::UInt(limit));
        }

        QueryAndValues(self.query, self.values)
    }
}

fn check_if_term_join_necessary(params: &Params) -> bool {
    params.term_and.is_some()
        || params.term_in.is_some()
        || params.term_not_in.is_some()
        || params.term_slug_and.is_some()
        || params.term_slug_in.is_some()
        || params.tax_query.is_some()
}

fn check_if_meta_join_necessary(params: &Params) -> bool {
    params.meta_key.is_some()
        || params.meta_value.is_some()
        || params.meta_value_num.is_some()
        || params.meta_query.is_some()
}

fn check_if_user_join_necessary(p: &Params) -> bool {
    p.author_name.is_some()
}

fn has_valid_single_date(p: &Params) -> bool {
    p.year.is_some() && p.monthnum.is_some() && p.day.is_some()
}

fn push_post_status(s: &mut String, v: &mut StmtValues, post_status: &PostStatus) {
    if *post_status == PostStatus::Any {
        return;
    }

    s.push_str(" AND post_status = ?");
    v.push(Value::Bytes(post_status.to_string().into_bytes()));
}

fn push_post_type(s: &mut String, v: &mut StmtValues, post_type: Option<Vec<PostType>>) {
    if let Some(post_types) = post_type {
        if post_types.len() == 0 {
            return;
        }

        let q_marks = implode_to_question_mark(&post_types);
        s.push_str(&format!(" AND wp_posts.post_type IN ({})", q_marks));
        for post_type in post_types {
            v.push(Value::Bytes(post_type.into()));
        }
    } else {
        s.push_str(" AND wp_posts.post_type = 'post'");
    }
}

fn implode<T: std::fmt::Display>(v: &[T]) -> String {
    v.iter()
        .map(|n| n.to_string())
        .reduce(|acc, next| format!("{acc},{next}"))
        .unwrap_or(String::new())
}

fn implode_to_question_mark<T>(v: &[T]) -> String {
    let q_marks: Vec<char> = v.iter().map(|_| '?').collect();
    implode(&q_marks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_implode_works() {
        let v = vec![1, 2, 3];
        let imploded = implode(&v);
        assert_eq!(&imploded, "1,2,3");
    }

    #[test]
    fn nothing_imploded_for_empty_list() {
        let v: Vec<i32> = vec![];
        let imploded = implode(&v);
        assert_eq!(&imploded, "");
    }

    #[test]
    fn implodes_to_question_marks() {
        let v = vec![1, 2, 3];
        let imploded = implode_to_question_mark(&v);
        assert_eq!(&imploded, "?,?,?");
    }
}
