use std::error::Error;

use mysql::prelude::Queryable;
use mysql_common::Row;
use query::params::Params;
use query_builder::QueryBuilder;
use sql::{get_pool, unwrap_row};

pub use param_builder::ParamBuilder;
pub use sql::SqlOrder;
pub use wp_post::WP_Post;
pub use wp_post::post_status::PostStatus;

pub mod param_builder;
pub mod query;
mod query_builder;
mod sql;
mod wp_post;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct WP_Query {
    pub posts: Vec<WP_Post>,
}

impl WP_Query {
    pub fn new(params: Params) -> Result<Self, Box<dyn Error>> {
        let pool = get_pool(sql::env_vars::EnvVars::from_env())?;

        let query_builder::QueryAndValues(q, values) = QueryBuilder::new(params).query()?;

        let mut conn = pool.get_conn()?;

        let stmt = conn.prep(q)?;
        let mut rows: Vec<Row> = conn.exec(stmt, values)?;
        let posts: Vec<WP_Post> = rows
            .iter_mut()
            .map(|row| unwrap_row(row))
            .filter_map(|row| row.ok())
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
