use std::error::Error;

use mysql_common::Value;
use sql_paginatorr::LimitOffsetPair;

use crate::{query::params::Params, sql::SqlOrder, wp_post::post_status::PostStatus};

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

    pub fn query(mut self) -> Result<QueryAndValues, Box<dyn Error>> {
        let params = &self.params;

        self.query.push_str(
            "SELECT DISTINCT(wp_posts.ID),post_author,comment_count,post_parent,menu_order,
            post_date,post_date_gmt,post_modified,post_modified_gmt,
            post_status,post_content,post_title,post_excerpt,comment_status,ping_status,
            post_password,post_name,to_ping,pinged,post_content_filtered,guid,
            post_type,post_mime_type
            FROM wp_posts",
        );

        let join_term = check_if_term_join_necessary(params);
        let join_meta = check_if_meta_join_necessary(params);
        let join_user = check_if_user_join_necessary(params);

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

        // Author conditions
        if let Some(author_id) = &params.author {
            self.query.push_str(" AND post_author = ?");
            self.values.push(Value::UInt(*author_id));
        }

        if let Some(author_name) = &params.author_name {
            self.query.push_str(" AND wp_users.user_nicename = ?");
            self.values
                .push(Value::Bytes(author_name.clone().into_bytes()));
        }

        if let Some(author_ids) = &params.author__in {
            let q_marks = implode_to_question_mark(author_ids);
            self.query.push_str(&format!(" AND post_author IN ({})", q_marks));
            let mut ids: Vec<Value> = author_ids.iter().map(|id| Value::UInt(*id)).collect();
            self.values.append(&mut ids);
        }

        if let Some(author_ids) = &params.author__not_in {
            let q_marks = implode_to_question_mark(author_ids);
            self.query.push_str(&format!(" AND post_author NOT IN ({})", q_marks));
            let mut ids: Vec<Value> = author_ids.iter().map(|id| Value::UInt(*id)).collect();
            self.values.append(&mut ids);
        }

        /* Add Post Status conditions */
        if let Some(post_status) = &params.post_status {
            push_post_status(&mut self.query, &mut self.values, &post_status);
        } else {
            self.query
                .push_str(&format!(" AND post_status = '{}'", PostStatus::Publish));
        }

        /* Add category conditions */
        if let Some(cat) = &params.cat {
            self.query.push_str(" AND wp_terms.term_id = ?");
            self.values.push(Value::UInt(*cat));
        }

        if let Some(cat) = &params.category_name {
            self.query.push_str(" AND wp_terms.slug = ?");
            self.values.push(Value::Bytes(cat.clone().into_bytes()));
        }

        if let Some(cat_ids) = &params.category__in {
            let q_marks = implode_to_question_mark(cat_ids);
            self.query.push_str(&format!(" AND wp_terms.term_id IN ({})", q_marks));
            let mut ids: Vec<Value> = cat_ids.iter().map(|id| Value::UInt(*id)).collect();
            self.values.append(&mut ids);
        }

        if let Some(cat_ids) = &params.category__not_in {
            let q_marks = implode_to_question_mark(cat_ids);
            self.query.push_str(&format!(" AND wp_terms.term_id NOT IN ({})", q_marks));
            let mut ids: Vec<Value> = cat_ids.iter().map(|id| Value::UInt(*id)).collect();
            self.values.append(&mut ids);
        }

        /* Add tag conditions */
        if let Some(tag) = &params.tag {
            self.query.push_str(" AND wp_terms.slug = ?");
            self.values.push(Value::Bytes(tag.clone().into_bytes()));
        }

        if let Some(orderby) = &params.orderby {
            let order = params.order.unwrap_or(SqlOrder::Desc).clone().to_string();
            self.query
                .push_str(&format!(" ORDER BY {} {}", orderby.to_string(), order))
        }

        if let Some(page) = params.page {
            let LimitOffsetPair { offset, limit } =
                sql_paginatorr::for_page(page as usize, params.posts_per_page.unwrap_or(10) as usize);

            self.query.push_str(" LIMIT ? OFFSET ?;");
            self.values.push(Value::UInt(limit as u64));
            self.values.push(Value::UInt(offset as u64));
        } else {
            self.query.push_str(" LIMIT 10;");
        }

        Ok(QueryAndValues(self.query, self.values))
    }
}

fn check_if_term_join_necessary(params: &Params) -> bool {
    params.tag.is_some()
        || params.tag__and.is_some()
        || params.tag__in.is_some()
        || params.tag__not_in.is_some()
        || params.tag_id.is_some()
        || params.tag_slug__and.is_some()
        || params.tag_slug__in.is_some()
        || params.tax_query.is_some()
        || params.cat.is_some()
        || params.category__and.is_some()
        || params.category__in.is_some()
        || params.category__not_in.is_some()
        || params.category_name.is_some()
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

fn push_post_status(s: &mut String, v: &mut StmtValues, post_status: &PostStatus) {
    if *post_status == PostStatus::Any {
        return;
    }

    s.push_str(" AND post_status = ?");
    v.push(Value::Bytes(post_status.to_string().into_bytes()));
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
