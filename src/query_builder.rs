use std::error::Error;

use mysql_common::Value;
use sql_paginatorr::LimitOffsetPair;

use crate::{query::params::Params, sql::SqlOrder};

pub struct QueryBuilder {
    params: Params,
    query: String,
    values: Vec<Value>,
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
            "SELECT DISTINCT(ID),post_author,comment_count,post_parent,menu_order,
            post_date,post_date_gmt,post_modified,post_modified_gmt,
            post_status,post_content,post_title,post_excerpt,comment_status,ping_status,
            post_password,post_name,to_ping,pinged,post_content_filtered,guid,
            post_type,post_mime_type
            FROM wp_posts",
        );

        let join_term = check_if_term_join_necessary(params);
        let join_meta = check_if_meta_join_necessary(params);

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

        if let Some(tag) = &params.tag {}

        if let Some(orderby) = &params.orderby {
            let order = params.order.unwrap_or(SqlOrder::Desc).clone().to_string();
            self.query
                .push_str(&format!(" ORDER BY {} {}", orderby.to_string(), order))
        }

        if let Some(page) = params.page {
            let LimitOffsetPair { offset, limit } =
                sql_paginatorr::for_page(page, params.posts_per_page.unwrap_or(10));

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
}

fn check_if_meta_join_necessary(params: &Params) -> bool {
    params.meta_key.is_some()
        || params.meta_value.is_some()
        || params.meta_value_num.is_some()
        || params.meta_query.is_some()
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
        let imploded = implode(&v);
        assert_eq!(&imploded, "?,?,?");
    }
}
