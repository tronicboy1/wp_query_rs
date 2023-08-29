use std::fmt::Display;

use mysql::prelude::Queryable;
use mysql_common::prelude::ToValue;

use crate::sql::get_conn;

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
    pub fn get_post_meta(post_id: u64, meta_key: &str, single: bool) -> WpMetaResults {
        let mut conn = get_conn().expect("CouldNotGetConnection");

        let stmt = conn
            .prep(
                "SELECT meta_id,post_id,meta_key,meta_value
                FROM wp_postmeta WHERE post_id = ? AND meta_key = ?;",
            )
            .expect("StmtError");
        let params = mysql::Params::Positional(vec![post_id.to_value(), meta_key.to_value()]);

        let to_wp_meta = |(meta_id, post_id, meta_key, meta_value)| WpMeta {
            meta_id,
            post_id,
            meta_value,
            meta_key,
        };

        if single {
            conn.exec_first(stmt, params)
                .ok()
                .flatten()
                .map(to_wp_meta)
                .map(|meta| WpMetaResults::Single(meta))
                .unwrap_or(WpMetaResults::Empty)
        } else {
            conn.exec_map(stmt, params, to_wp_meta)
                .map(|postmeta| WpMetaResults::Array(postmeta))
                .unwrap_or(WpMetaResults::Empty)
        }
    }

    pub fn add_post_meta<T>(post_id: u64, meta_key: &str, meta_value: T) -> Result<(), mysql::Error>
    where
        T: Display,
    {
        let mut conn = get_conn()?;

        let stmt = conn.prep(
            "INSERT INTO wp_postmeta (
            post_id,
            meta_key,
            meta_value
        ) VALUES (?, ?, ?);",
        )?;

        let meta_value = format!("{}", meta_value);

        conn.exec_drop(stmt, (post_id, meta_key, meta_value))?;

        Ok(())
    }
}
