use mysql::prelude::Queryable;
use mysql::PooledConn;
use mysql_common::Row;
use params::Params;
use query_builder::QueryBuilder;
use sql::get_conn;
use sql::unwrap_row;
use std::error::Error;

pub use params::date_query::DateColumn;
pub use params::date_query::DateQuery;
pub use params::date_query::DateQueryAfterBefore;
pub use params::meta_query::MetaQuery;
pub use params::meta_query::MetaRelation;
pub use params::orderby::WpOrderBy;
pub use params::param_builder::ParamBuilder;
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
    /**
     * Queries the WordPress Database for posts.
     *
     * Uses environment variables to get a connection to the database.
     *
     * For using your own pool/connection, use `WP_Query::with_connection`.
     */
    pub fn new(params: Params) -> Result<Self, Box<dyn Error>> {
        let mut conn = get_conn()?;

        let posts: Vec<WP_Post> = Self::query(&mut conn, params)?;

        Ok(Self { posts })
    }

    /**
     * Queries the WordPress database with a connection provided
     */
    pub fn with_connection(conn: &mut PooledConn, params: Params) -> Result<Self, Box<dyn Error>> {
        let posts: Vec<WP_Post> = Self::query(conn, params)?;

        Ok(Self { posts })
    }

    fn query(conn: &mut PooledConn, params: Params) -> Result<Vec<WP_Post>, Box<dyn Error>> {
        let query_builder::QueryAndValues(q, values) = QueryBuilder::new(params).query()?;

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
