//! # WP Query Rust
//! A rust implementation of the classic WP_Query utility to access WordPress posts outside of a WordPress environment.

use mysql::prelude::Queryable;
use mysql::PooledConn;
use mysql_common::Row;
use query_builder::QueryBuilder;
use sql::get_conn;
use sql::unwrap_row;

pub use params::date_query::DateColumn;
pub use params::date_query::DateQuery;
pub use params::date_query::DateQueryAfterBefore;
pub use params::meta_query::MetaQuery;
pub use params::meta_query::MetaRelation;
pub use params::orderby::WpOrderBy;
pub use params::param_builder::ParamBuilder;
pub use params::Params;
pub use sql::env_vars::EnvVars;
pub use sql::SqlOrder;
pub use sql::SqlSearchOperators;
pub use wp_post::post_status::PostStatus;
pub use wp_post::WP_Post;

mod params;
mod query_builder;
mod sql;
mod wp_post;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct WP_Query {
    pub posts: Vec<WP_Post>,
}

impl WP_Query {
    /// Queries the WordPress Database for posts.
    ///
    /// Uses environment variables to create a connection pool that is shared through the life of the application ('static).
    ///
    /// # Example
    ///
    /// ```
    /// use wp_query_rs::{ParamBuilder, WP_Query};
    ///
    /// let params = ParamBuilder::new().page(1).post_type("post");
    ///
    /// let wp_q = WP_Query::new(params);
    /// ```
    ///
    /// # Errors
    /// Will return an error if there is an error in the mysql query. This may be from innapropriate SQL built in the query builder,
    /// or more likely a connection issue from incorrect environment variables.
    pub fn new(params: Params) -> Result<Self, mysql::Error> {
        let mut conn = get_conn()?;

        let posts: Vec<WP_Post> = Self::query(&mut conn, params)?;

        Ok(Self { posts })
    }

    /// Queries the WordPress database with a connection provided from a mysql connection pool.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use wp_query_rs::{ParamBuilder, WP_Query};
    ///
    /// let my_pool: mysql::Pool;
    ///
    /// let mut conn = my_pool.get_conn().unwrap();
    ///
    /// let params = ParamBuilder::new().page(1).post_type("post");
    ///
    /// let wp_q = WP_Query::with_connection(&mut conn, params);
    /// ```
    ///
    /// # Errors
    /// When an error occurs in the SQL query.
    pub fn with_connection(conn: &mut PooledConn, params: Params) -> Result<Self, mysql::Error> {
        let posts: Vec<WP_Post> = Self::query(conn, params)?;

        Ok(Self { posts })
    }

    fn query(conn: &mut PooledConn, params: Params) -> Result<Vec<WP_Post>, mysql::Error> {
        let query_builder::QueryAndValues(q, values) = QueryBuilder::new(params).query();

        let stmt = conn.prep(q)?;

        let mut rows: Vec<Row> = conn.exec(stmt, values)?;

        Ok(rows
            .iter_mut()
            .map(|row| unwrap_row(row))
            .filter_map(|row| row.ok())
            .collect())
    }

    pub fn post_count(&self) -> usize {
        self.posts.len()
    }

    fn max_num_pages(&self) -> usize {
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
