use std::{error::Error, str::FromStr};

use mysql::prelude::Queryable;
use mysql_common::{time::Date, Row};
use query::params::Params;
use query_builder::QueryBuilder;
use sql::get_pool;
use wp_post::WP_Post;

pub mod param_builder;
mod query;
mod query_builder;
mod sql;
mod wp_post;

pub use param_builder::ParamBuilder;

use crate::wp_post::post_status::PostStatus;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct WP_Query {
    pub posts: Vec<WP_Post>,
}

impl WP_Query {
    pub fn new(params: Params) -> Result<Self, Box<dyn Error>> {
        let pool = get_pool(sql::env_vars::EnvVars::from_env())?;

        let q = QueryBuilder::new(params).query()?;

        let mut conn = pool.get_conn()?;

        let mut rows: Vec<Row> = conn.exec(q, ())?;
        let posts: Vec<WP_Post> = rows
            .iter_mut()
            .map(|row| {
                let id: u32 = row.take(0).unwrap();
                let post_author: u32 = row.take(1).unwrap();
                let comment_count: u32 = row.take(2).unwrap();
                let post_parent: u32 = row.take(3).unwrap();
                let menu_order: u32 = row.take(4).unwrap();
                let post_date: Date = row.take(5).unwrap();
                let post_date_gmt: Date = row.take(6).unwrap();
                let post_modified: Date = row.take(7).unwrap();
                let post_modified_gmt: Date = row.take(8).unwrap();
                let post_status: String = row.take(9).unwrap();
                let post_status = PostStatus::from_str(&post_status).unwrap();
                let post_content: String = row.take(10).unwrap();
                let post_title: String = row.take(11).unwrap();
                let post_excerpt: String = row.take(12).unwrap();
                let comment_status: String = row.take(13).unwrap();
                let ping_status: String = row.take(14).unwrap();
                let post_password: String = row.take(15).unwrap();
                let post_name: String = row.take(16).unwrap();
                let to_ping: String = row.take(17).unwrap();
                let pinged: String = row.take(18).unwrap();
                let post_content_filtered: String = row.take(19).unwrap();
                let guid: String = row.take(20).unwrap();
                let post_type: String = row.take(21).unwrap();
                let post_mime_type: String = row.take(22).unwrap();

                WP_Post {
                    ID: id,
                    post_author,
                    post_date,
                    post_date_gmt,
                    post_content,
                    post_title,
                    post_excerpt,
                    post_status,
                    comment_status,
                    ping_status,
                    post_password,
                    post_name,
                    to_ping,
                    pinged,
                    post_modified,
                    post_modified_gmt,
                    post_content_filtered,
                    post_parent,
                    guid,
                    menu_order,
                    post_type,
                    post_mime_type,
                    comment_count,
                }
            })
            .collect();

        Ok(Self { posts })
    }

    pub fn post_count(&self) -> usize {
        self.posts.len()
    }

    pub fn max_num_pages(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
