//! # WP Query Rust
//! A rust implementation of the classic WP_Query utility to access WordPress posts outside of a WordPress environment.

use mysql::prelude::Queryable;
use mysql_common::Row;
use query_builder::QueryBuilder;
use sql::get_conn;

pub use params::date_query::DateColumn;
pub use params::date_query::DateQuery;
pub use params::date_query::DateQueryAfterBefore;
pub use params::meta_query::MetaQuery;
pub use params::meta_query::MetaRelation;
pub use params::orderby::WpOrderBy;
pub use params::param_builder::ParamBuilder;
pub use params::post_type::PostType;
pub use params::tax_query::TaxField;
pub use params::tax_query::TaxQuery;
pub use params::tax_query::TaxRelation;
pub use params::traits::*;
pub use params::Params;
pub use sql::env_vars::EnvVars;
pub use sql::traits::Insertable;
pub use sql::SqlOrder;
pub use sql::SqlSearchOperators;
pub use wp_post::add_post_meta;
pub use wp_post::get_post_meta;
pub use wp_post::post_status::PostStatus;
use wp_post::WpPost;
pub use wp_user::WpUser;

// TODO remove on next major version
#[allow(non_camel_case_types)]
pub type WP_Post = WpPost;
#[allow(non_camel_case_types)]
pub type WP_Query = WpQuery;

mod params;
mod query_builder;
mod sql;
mod wp_comment;
pub mod wp_post;
mod wp_user;

#[derive(Debug)]
pub struct WpQuery {
    pub posts: Vec<WpPost>,
}

impl WpQuery {
    /// Queries the WordPress Database for posts.
    ///
    /// Uses environment variables to create a connection pool that is shared through the life of the application ('static).
    ///
    /// # Example
    ///
    /// ```
    /// use wp_query_rs::{ParamBuilder, WP_Query, PostType, PostQueryable};
    ///
    /// let params = ParamBuilder::new().page(1).post_type(PostType::Post);
    ///
    /// let wp_q = WP_Query::new(params);
    /// ```
    ///
    /// # Errors
    /// Will return an error if there is an error in the mysql query. This may be from innapropriate SQL built in the query builder,
    /// or more likely a connection issue from incorrect environment variables.
    pub fn new<'a, T>(params: T) -> Result<Self, mysql::Error>
    where
        T: Into<Params<'a>>,
    {
        let mut conn = get_conn()?;

        let posts: Vec<WpPost> = Self::query(&mut conn, params)?;

        Ok(Self { posts })
    }

    /// Queries the WordPress database with a mysql connection.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use wp_query_rs::{ParamBuilder, WP_Query, PostType, PostQueryable};
    ///
    /// let my_pool: mysql::Pool;
    ///
    /// let mut conn = my_pool.get_conn().unwrap();
    ///
    /// let params = ParamBuilder::new().page(1).post_type(PostType::Post);
    ///
    /// let wp_q = WP_Query::with_connection(&mut conn, params);
    /// ```
    ///
    /// # Errors
    /// When an error occurs in the SQL query.
    pub fn with_connection<'a, T>(
        conn: &mut impl Queryable,
        params: T,
    ) -> Result<Self, mysql::Error>
    where
        T: Into<Params<'a>>,
    {
        let posts: Vec<WpPost> = Self::query(conn, params)?;

        Ok(Self { posts })
    }

    fn query<'a, T>(conn: &mut impl Queryable, params: T) -> Result<Vec<WpPost>, mysql::Error>
    where
        T: Into<Params<'a>>,
    {
        let query_builder::QueryAndValues(q, values) = QueryBuilder::new(params.into()).query();

        let stmt = conn.prep(q)?;

        let rows: Vec<Row> = conn.exec(stmt, values)?;

        Ok(rows.into_iter().map(|row| WpPost::from(row)).collect())
    }

    pub fn post_count(&self) -> usize {
        self.posts.len()
    }

    fn _max_num_pages(&self) -> usize {
        0
    }

    pub fn to_vec(self) -> Vec<WpPost> {
        self.posts
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
