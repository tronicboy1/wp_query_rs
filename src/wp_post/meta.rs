use std::{fmt::Display, vec};

use mysql::prelude::Queryable;
use mysql_common::prelude::ToValue;

use crate::{
    ok_or_row_error,
    sql::{find_col, get_conn, traits::Insertable},
};

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
        let params = mysql::Params::Positional(vec![post_id.to_value(), meta_key.to_value()]);

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

    fn prepare_insert_stmt(conn: &mut impl Queryable) -> Result<mysql::Statement, mysql::Error> {
        conn.prep(
            "INSERT INTO wp_postmeta (
            meta_id,
            post_id,
            meta_key,
            meta_value
        ) VALUES (?, ?, ?, ?);",
        )
    }

    /// Allows multiple meta to be added in the same prepared statement, improving speed
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

impl Into<mysql::Params> for WpMeta {
    fn into(self) -> mysql::Params {
        mysql::Params::Positional(vec![
            self.meta_id.to_value(),
            self.post_id.to_value(),
            self.meta_key.to_value(),
            self.meta_value.to_value(),
        ])
    }
}

impl Insertable for WpMeta {
    fn insert(self) -> Result<u64, mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::prepare_insert_stmt(&mut conn)?;

        conn.exec_drop(stmt, self)?;

        Ok(conn.exec_first("SELECT LAST_INSERT_ID();", ())?.unwrap())
    }

    fn batch(values: impl IntoIterator<Item = Self>) -> Result<(), mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::prepare_insert_stmt(&mut conn)?;

        conn.exec_batch(stmt, values)?;

        Ok(())
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
