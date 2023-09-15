use mysql_common::{prelude::ToValue, Value};
use sql_paginatorr::LimitOffsetPair;

use crate::{params::Params, sql::SqlOrder, wp_post::post_status::PostStatus};

type StmtValues = Vec<Value>;

pub struct QueryBuilder {
    params: Params,
    query: String,
    values: StmtValues,
}

pub struct QueryAndValues(pub String, pub Vec<Value>);

impl QueryBuilder {
    pub fn new(params: Params) -> Self {
        Self {
            params,
            query: String::new(),
            values: vec![],
        }
    }

    pub fn query(mut self) -> QueryAndValues {
        let params = self.params;

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
            INNER JOIN wp_terms ON wp_terms.term_id = wp_term_relationships.term_taxonomy_id",
            );
        }

        if join_user {
            self.query
                .push_str(" INNER JOIN wp_users ON wp_users.ID = wp_posts.post_author");
        }

        // Avoid dangling WHERE issue
        self.query.push_str(" WHERE 1 = 1");

        /* Author conditions */
        if let Some(author_id) = params.author {
            self.query.push_str(" AND post_author = ?");
            self.values.push(Value::UInt(author_id));
        }

        if let Some(author_name) = params.author_name {
            self.query.push_str(" AND wp_users.user_nicename = ?");
            self.values.push(Value::Bytes(author_name.into_bytes()));
        }

        if let Some(author_ids) = params.author__in {
            let q_marks = implode_to_question_mark(&author_ids);
            self.query
                .push_str(&format!(" AND post_author IN ({})", q_marks));
            let mut ids: Vec<Value> = author_ids.into_iter().map(|id| Value::UInt(id)).collect();
            self.values.append(&mut ids);
        }

        if let Some(author_ids) = params.author__not_in {
            let q_marks = implode_to_question_mark(&author_ids);
            self.query
                .push_str(&format!(" AND post_author NOT IN ({})", q_marks));
            let mut ids: Vec<Value> = author_ids.into_iter().map(|id| Value::UInt(id)).collect();
            self.values.append(&mut ids);
        }

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
                self.values.push(Value::Bytes(term_slug.into_bytes()));
            }
        }

        if let Some(term_ids) = params.term_in {
            let q_marks = implode_to_question_mark(&term_ids);
            self.query
                .push_str(&format!(" AND wp_terms.term_id IN ({})", q_marks));
            let mut ids: Vec<Value> = term_ids.into_iter().map(|id| Value::UInt(id)).collect();
            self.values.append(&mut ids);
        }

        if let Some(term_ids) = params.term_not_in {
            let q_marks = implode_to_question_mark(&term_ids);
            self.query
                .push_str(&format!(" AND wp_terms.term_id NOT IN ({})", q_marks));
            let mut ids: Vec<Value> = term_ids.into_iter().map(|id| Value::UInt(id)).collect();
            self.values.append(&mut ids);
        }

        /* Add search conditions */
        if let Some(keyword) = params.s {
            self.query
                .push_str(" AND wp_posts.post_content LIKE CONCAT('%',?,'%') OR wp_posts.post_title LIKE CONCAT('%',?,'%')");
            self.values.push(Value::Bytes(keyword.as_bytes().to_vec())); // Clone this so it can be used again
            self.values.push(Value::Bytes(keyword.into_bytes()));
        }

        /* Add page/post conditions */
        if let Some(p_id) = params.p {
            self.query.push_str(" AND wp_posts.ID = ?");
            self.values.push(Value::UInt(p_id));
        }

        if let Some(name) = params.name {
            self.query.push_str(" AND wp_posts.post_name = ?");
            self.values.push(Value::Bytes(name.into_bytes()));
        }

        /* Post types */
        push_post_type(&mut self.query, &mut self.values, params.post_type);

        if let Some(par_id) = params.post_parent {
            self.query.push_str(" AND wp_posts.post_parent = ?");
            self.values.push(Value::UInt(par_id));
        }

        if let Some(p_ids) = params.post_parent__in {
            let q_marks = implode_to_question_mark(&p_ids);
            self.query
                .push_str(&format!(" AND wp_posts.post_parent IN ({})", q_marks));
            let mut ids: Vec<Value> = p_ids.into_iter().map(|id| Value::UInt(id)).collect();
            self.values.append(&mut ids);
        }

        if let Some(p_ids) = params.post_parent__not_in {
            let q_marks = implode_to_question_mark(&p_ids);
            self.query
                .push_str(&format!(" AND wp_posts.post_parent NOT IN ({})", q_marks));
            let mut ids: Vec<Value> = p_ids.into_iter().map(|id| Value::UInt(id)).collect();
            self.values.append(&mut ids);
        }

        if let Some(p_ids) = params.post__in {
            let q_marks = implode_to_question_mark(&p_ids);
            self.query
                .push_str(&format!(" AND wp_posts.ID IN ({})", q_marks));
            let mut ids: Vec<Value> = p_ids.iter().map(|id| Value::UInt(*id)).collect();
            self.values.append(&mut ids);
        }

        if let Some(p_ids) = params.post__not_in {
            let q_marks = implode_to_question_mark(&p_ids);
            self.query
                .push_str(&format!(" AND wp_posts.ID NOT IN ({})", q_marks));
            let mut ids: Vec<Value> = p_ids.iter().map(|id| Value::UInt(*id)).collect();
            self.values.append(&mut ids);
        }

        if let Some(p_names) = params.post_name__in {
            let q_marks = implode_to_question_mark(&p_names);
            self.query
                .push_str(&format!(" AND wp_posts.post_name IN ({})", q_marks));
            let mut ids: Vec<Value> = p_names
                .into_iter()
                .map(|name| Value::Bytes(name.into_bytes()))
                .collect();
            self.values.append(&mut ids);
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
                if let Some(after) = &dq.after {
                    let d_op = if dq.inclusive { ">=" } else { ">" };
                    self.query
                        .push_str(&format!(" {} wp_posts.{} {} ?", &op, col, d_op));
                    self.values.push(after.to_value());
                }

                if let Some(before) = &dq.before {
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
            self.values.push(Value::Bytes(meta_k.into_bytes()));
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

                    let op = query.compare.to_string();
                    self.query.push_str(
                        format!(
                            " (wp_postmeta.meta_value {} ? AND wp_postmeta.meta_key {} ?)",
                            op, op
                        )
                        .as_str(),
                    );
                    self.values.push(Value::Bytes(query.value.into_bytes()));
                    self.values.push(Value::Bytes(query.key.into_bytes()));
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

fn push_post_type(s: &mut String, v: &mut StmtValues, post_type: Option<Vec<String>>) {
    if let Some(post_types) = post_type {
        if post_types.len() == 0 {
            return;
        }

        let q_marks = implode_to_question_mark(&post_types);
        s.push_str(&format!(" AND wp_posts.post_type IN ({})", q_marks));
        for post_type in post_types {
            v.push(Value::Bytes(post_type.into_bytes()));
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
