//! # WP Query Rust
//! A rust implementation of the classic WP_Query utility to access WordPress posts outside of a WordPress environment.
//!
//! # Features
//!
//! ## query_sync
//!
//! Adds default functionality to connect to the database using a global connection pool initialized when WpQuery::new
//! or any other internal synchronous mysql function is called. This uses the synchronous blocking logic.
//!
//! This feature can be paired with native-tls or rustls depending on environment needs.
//!
//! ## query_async
//!
//! **TODO**
//! Use asynchronous mysql logic to query database.
//!
//! ## native-tls
//!
//! Sets the mysql crate feature to use native TLS.
//!
//! ## rustls
//!
//! Sets the mysql crate to use rustls (non-native). This helps when compiling code for say a lambda function where you do not
//! have access to native TLS.
//!
//! ## php
//!
//! **WIP**
//! Adds some serialization for use with ext_php_rs.
//!
//! ## rewrite
//!
//! **WIP**
//! Adds ability to parse pretty URLs from wordpress rewrite settings. Currently we can only parse urls into parameters,
//! not generate PHP serialized rewrite rules to be used in WordPress.

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
use query_builder::QueryBuilder;
#[cfg(any(feature = "query_sync", feature = "query_async"))]
use sql::get_conn;
pub use sql::SqlOrder;
pub use sql::SqlSearchOperators;
pub use wp_post::post_status::PostStatus;
use wp_post::WpPost;
pub use wp_user::WpUser;

#[cfg(feature = "query_sync")]
use mysql::prelude::Queryable;
#[cfg(feature = "query_sync")]
pub use sql::env_vars::EnvVars;
#[cfg(feature = "query_sync")]
pub use sql::traits::Insertable;
#[cfg(feature = "query_sync")]
pub use wp_post::add_post_meta;
#[cfg(feature = "query_sync")]
pub use wp_post::get_post_meta;

#[cfg(feature = "query_async")]
use mysql_async::prelude::*;

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

#[cfg(feature = "rewrite")]
pub mod wp_rewrite;

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
    #[cfg(feature = "query_sync")]
    pub fn new<'a, T>(params: T) -> Result<Self, mysql::Error>
    where
        T: Into<Params<'a>>,
    {
        let mut conn = get_conn()?;

        let posts: Vec<WpPost> = Self::query(&mut conn, params)?;

        Ok(Self { posts })
    }
    #[cfg(feature = "query_async")]
    pub async fn new<'a, T>(params: T) -> Result<Self, mysql_async::Error>
    where
        T: Into<Params<'a>>,
    {
        let mut conn = get_conn().await?;

        let posts: Vec<WpPost> = Self::query(&mut conn, params).await?;

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
    #[cfg(feature = "query_sync")]
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
    #[cfg(feature = "query_async")]
    pub async fn with_connection<'a, T>(
        conn: &mut mysql_async::Conn,
        params: T,
    ) -> Result<Self, mysql_async::Error>
    where
        T: Into<Params<'a>>,
    {
        let posts = Self::query(conn, params).await?;

        Ok(Self { posts })
    }

    #[cfg(feature = "query_sync")]
    fn query<'a, T>(conn: &mut impl Queryable, params: T) -> Result<Vec<WpPost>, mysql::Error>
    where
        T: Into<Params<'a>>,
    {
        let query_builder::QueryAndValues(q, values) = QueryBuilder::new(params.into()).query();

        let stmt = conn.prep(q)?;

        conn.exec(stmt, values)
    }
    #[cfg(feature = "query_async")]
    async fn query<'a, T>(
        conn: &mut mysql_async::Conn,
        params: T,
    ) -> Result<Vec<WpPost>, mysql_async::Error>
    where
        T: Into<Params<'a>>,
    {
        let query_builder::QueryAndValues(q, values) = QueryBuilder::new(params.into()).query();

        let stmt = conn.prep(q).await?;

        conn.exec(stmt, values).await
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
