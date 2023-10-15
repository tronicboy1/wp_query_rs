#[cfg(feature = "query_sync")]
use mysql::prelude::*;
#[cfg(feature = "query_async")]
use mysql_async::prelude::*;
use mysql_common::params::Params;
#[cfg(any(feature = "query_sync", feature = "query_async"))]
use std::{fmt::Display, vec};

#[cfg(any(feature = "query_sync", feature = "query_async"))]
use crate::sql::{get_conn, traits::Insertable};
use crate::{ok_or_row_error, sql::find_col};

#[derive(Debug)]
pub struct WpMeta {
    meta_id: u64,
    post_id: u64,
    pub meta_value: String,
    pub meta_key: String,
}

#[derive(Debug)]
pub enum WpMetaResults {
    Single(WpMeta),
    Array(Vec<WpMeta>),
    Empty,
}

macro_rules! prepare_insert_stmt {
    ($conn: ty, $result: ty) => {
        fn prepare_insert_stmt(conn: &mut $conn) -> $result {
            conn.prep(
                "INSERT INTO wp_postmeta (
                meta_id,
                post_id,
                meta_key,
                meta_value
            ) VALUES (?, ?, ?, ?);",
            )
        }
    };
}

impl WpMeta {
    pub fn new<T>(post_id: u64, meta_key: &str, meta_value: T) -> Self
    where
        T: ToString,
    {
        Self {
            meta_id: 0,
            post_id,
            meta_value: meta_value.to_string(),
            meta_key: meta_key.to_string(),
        }
    }

    #[cfg(feature = "query_sync")]
    pub fn get_post_meta(
        post_id: u64,
        meta_key: &str,
        single: bool,
    ) -> Result<WpMetaResults, mysql::Error> {
        let mut conn = get_conn().expect("CouldNotGetConnection");

        let stmt = conn
            .prep(
                "SELECT * FROM wp_postmeta
                WHERE post_id = ? AND meta_key = ?;",
            )
            .expect("StmtError");
        let params = Params::Positional(vec![post_id.to_value(), meta_key.to_value()]);

        if single {
            conn.exec_first(stmt, params).map(|meta| match meta {
                Some(meta) => WpMetaResults::Single(meta),
                None => WpMetaResults::Empty,
            })
        } else {
            conn.exec(stmt, params)
                .map(|postmeta| WpMetaResults::Array(postmeta))
        }
    }
    #[cfg(feature = "query_async")]
    pub async fn get_post_meta(
        post_id: u64,
        meta_key: &str,
        single: bool,
    ) -> Result<WpMetaResults, mysql_async::Error> {
        let mut conn = get_conn().await?;

        let stmt = conn
            .prep(
                "SELECT * FROM wp_postmeta
                WHERE post_id = ? AND meta_key = ?;",
            )
            .await?;
        let params = Params::Positional(vec![post_id.to_value(), meta_key.to_value()]);

        if single {
            conn.exec_first(stmt, params).await.map(|meta| match meta {
                Some(meta) => WpMetaResults::Single(meta),
                None => WpMetaResults::Empty,
            })
        } else {
            conn.exec(stmt, params)
                .await
                .map(|postmeta| WpMetaResults::Array(postmeta))
        }
    }

    #[cfg(feature = "query_sync")]
    pub fn add_post_meta<T>(
        post_id: u64,
        meta_key: &str,
        meta_value: T,
    ) -> Result<u64, mysql::Error>
    where
        T: Display,
    {
        Self::insert(Self::new(post_id, meta_key, meta_value))
    }
    #[cfg(feature = "query_async")]
    pub async fn add_post_meta<T>(
        post_id: u64,
        meta_key: &str,
        meta_value: T,
    ) -> Result<u64, mysql_async::Error>
    where
        T: Display,
    {
        Self::insert(Self::new(post_id, meta_key, meta_value)).await
    }

    #[cfg(feature = "query_sync")]
    prepare_insert_stmt!(impl Queryable, Result<mysql::Statement, mysql::Error>);

    #[cfg(feature = "query_async")]
    prepare_insert_stmt!(
        mysql_async::Conn,
        std::pin::Pin<
            Box<
                dyn std::future::Future<Output = Result<mysql_async::Statement, mysql_async::Error>>
                    // Lives as long at least as the Conn reference!
                    + '_,
            >,
        >
    );

    /// Allows multiple meta to be added in the same prepared statement, improving speed
    #[cfg(feature = "query_sync")]
    pub fn add_post_meta_bulk<T>(
        post_id: u64,
        meta_key_value_pairs: &[(&str, T)],
    ) -> Result<(), mysql::Error>
    where
        T: Display,
    {
        let values = meta_key_value_pairs
            .iter()
            .map(|(meta_key, meta_value)| WpMeta::new(post_id, meta_key, meta_value));

        Self::batch(values)
    }
    #[cfg(feature = "query_async")]
    pub async fn add_post_meta_bulk<T>(
        post_id: u64,
        meta_key_value_pairs: &[(&str, T)],
    ) -> Result<(), mysql_async::Error>
    where
        T: Display,
    {
        // Must collect to vector so it can be possibly sent in async runtime
        let values: Vec<_> = meta_key_value_pairs
            .iter()
            .map(|(meta_key, meta_value)| WpMeta::new(post_id, meta_key, meta_value))
            .collect();

        Self::batch(values).await
    }
}

impl mysql_common::prelude::FromRow for WpMeta {
    fn from_row_opt(mut row: mysql_common::Row) -> Result<Self, mysql_common::FromRowError>
    where
        Self: Sized,
    {
        let meta = Self {
            meta_id: ok_or_row_error!(row, "meta_id"),
            post_id: ok_or_row_error!(row, "post_id"),
            meta_value: ok_or_row_error!(row, "meta_value"),
            meta_key: ok_or_row_error!(row, "meta_key"),
        };

        Ok(meta)
    }
}

#[cfg(any(feature = "query_sync", feature = "query_async"))]
impl Into<mysql_common::params::Params> for WpMeta {
    fn into(self) -> mysql_common::params::Params {
        mysql_common::params::Params::Positional(vec![
            self.meta_id.to_value(),
            self.post_id.to_value(),
            self.meta_key.to_value(),
            self.meta_value.to_value(),
        ])
    }
}

#[cfg(any(feature = "query_sync", feature = "query_async"))]
impl Insertable for WpMeta {
    #[cfg(feature = "query_sync")]
    fn insert(self) -> Result<u64, mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::prepare_insert_stmt(&mut conn)?;

        conn.exec_drop(stmt, self)?;

        Ok(conn.exec_first("SELECT LAST_INSERT_ID();", ())?.unwrap())
    }

    #[cfg(feature = "query_sync")]
    fn batch(values: impl IntoIterator<Item = Self>) -> Result<(), mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::prepare_insert_stmt(&mut conn)?;

        conn.exec_batch(stmt, values)?;

        Ok(())
    }

    #[cfg(feature = "query_async")]
    fn batch<T>(
        values: T,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), mysql_async::Error>>>>
    where
        T: IntoIterator<Item = Self> + Send + 'static,
        T::IntoIter: Send,
        Self: Sized,
    {
        let fut = async {
            let mut conn = get_conn().await?;

            let stmt = Self::prepare_insert_stmt(&mut conn).await?;

            conn.exec_batch(stmt, values).await?;

            Ok(())
        };

        Box::pin(fut)
    }

    #[cfg(feature = "query_async")]
    fn insert(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<u64, mysql_async::Error>>>> {
        let fut = async {
            let mut conn = get_conn().await?;

            let stmt = Self::prepare_insert_stmt(&mut conn).await?;

            conn.exec_drop(stmt, self).await?;

            Ok(conn
                .exec_first("SELECT LAST_INSERT_ID();", ())
                .await?
                .unwrap())
        };

        Box::pin(fut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_with_number() {
        let meta = WpMeta::new(1, "my_metakey", 1);
        assert_eq!(meta.post_id, 1);
        assert_eq!(meta.meta_key, String::from("my_metakey"));
        assert_eq!(meta.meta_id, 0);
        assert_eq!(meta.meta_value, String::from("1"));
    }

    #[test]
    fn can_create_with_str() {
        let meta = WpMeta::new(1, "my_metakey", "1");
        assert_eq!(meta.post_id, 1);
        assert_eq!(meta.meta_key, String::from("my_metakey"));
        assert_eq!(meta.meta_id, 0);
        assert_eq!(meta.meta_value, String::from("1"));
    }
}
