use std::{ops::Deref, str::FromStr};

use mysql::{prelude::Queryable, Statement};
use mysql_common::prelude::ToValue;

use crate::{sql::get_conn, Insertable};

use super::{get_date_now, get_utc_date_now, WP_Post};

impl WP_Post {
    fn get_stmt(conn: &mut impl Queryable) -> Result<Statement, mysql::Error> {
        conn.prep(
            "INSERT INTO `wp_posts` (
            /* For new posts, ID will be 0 so MySQL will create an ID for us */
            `ID`,
            `post_author`,
            `post_date`,
            `post_date_gmt`,
            `post_content`,
            `post_title`,
            `post_excerpt`,
            `post_status`,
            `comment_status`,
            `ping_status`,
            `post_password`,
            `post_name`,
            `to_ping`,
            `pinged`,
            `post_modified`,
            `post_modified_gmt`,
            `post_content_filtered`,
            `post_parent`,
            `guid`,
            `menu_order`,
            `post_type`,
            `post_mime_type`,
            `comment_count`
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
        )
    }

    pub fn insert(self) -> Result<u64, mysql::Error> {
        <Self as Insertable>::insert(self)
    }

    pub fn insert_bulk(v: Vec<Self>) -> Result<(), mysql::Error> {
        <Self as Insertable>::batch(v)
    }
}

impl Into<mysql::Params> for WP_Post {
    fn into(self) -> mysql::Params {
        mysql::Params::Positional(vec![
            self.ID.to_value(),
            self.post_author.to_value(),
            self.post_date.to_value(),
            self.post_date_gmt.to_value(),
            self.post_content.to_value(),
            self.post_title.to_value(),
            self.post_excerpt.to_value(),
            self.post_status.to_value(),
            self.comment_status.to_value(),
            self.ping_status.to_value(),
            self.post_password.to_value(),
            self.post_name.to_value(),
            self.to_ping.to_value(),
            self.pinged.to_value(),
            self.post_modified.to_value(),
            self.post_modified_gmt.to_value(),
            self.post_content_filtered.to_value(),
            self.post_parent.to_value(),
            self.guid.to_value(),
            self.menu_order.to_value(),
            self.post_type.to_value(),
            self.post_mime_type.to_value(),
            self.comment_count.to_value(),
        ])
    }
}

impl From<mysql::Row> for WP_Post {
    fn from(mut row: mysql::Row) -> Self {
        let mut post = WP_Post::new(0);

        let cols = row.columns();
        let col_names_i = cols.iter().enumerate().map(|(i, col)| (i, col.name_str()));

        for (i, column) in col_names_i {
            match column.deref() {
                "ID" => post.ID = row.take(i).unwrap(),
                "post_author" => post.post_author = row.take(i).unwrap(),
                "post_date" => post.post_date = row.take_opt(i).unwrap().unwrap_or(get_date_now()),
                "post_date_gmt" => {
                    post.post_date_gmt = row.take_opt(i).unwrap().unwrap_or(get_utc_date_now())
                }
                "post_content" => post.post_content = row.take(i).unwrap(),
                "post_title" => post.post_title = row.take(i).unwrap(),
                "post_excerpt" => post.post_excerpt = row.take(i).unwrap(),
                "post_status" => {
                    let s: String = row.take(i).unwrap();
                    post.post_status = super::PostStatus::from_str(&s).unwrap()
                }
                "comment_status" => post.comment_status = row.take(i).unwrap(),
                "ping_status" => post.ping_status = row.take(i).unwrap(),
                "post_password" => post.post_password = row.take(i).unwrap(),
                "post_name" => post.post_name = row.take(i).unwrap(),
                "to_ping" => post.to_ping = row.take(i).unwrap(),
                "pinged" => post.pinged = row.take(i).unwrap(),
                "post_modified" => {
                    post.post_modified = row.take_opt(i).unwrap().unwrap_or(get_date_now())
                }
                "post_modified_gmt" => {
                    post.post_modified_gmt = row.take_opt(i).unwrap().unwrap_or(get_utc_date_now())
                }
                "post_content_filtered" => post.post_content_filtered = row.take(i).unwrap(),
                "post_parent" => post.post_parent = row.take(i).unwrap(),
                "guid" => post.guid = row.take(i).unwrap(),
                "menu_order" => post.menu_order = row.take(i).unwrap(),
                "post_type" => post.post_type = row.take(i).unwrap(),
                "post_mime_type" => post.post_mime_type = row.take(i).unwrap(),
                "comment_count" => post.comment_count = row.take(i).unwrap(),
                _ => {}
            }
        }

        post
    }
}

impl Insertable for WP_Post {
    fn batch(values: impl IntoIterator<Item = Self>) -> Result<(), mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::get_stmt(&mut conn)?;

        conn.exec_batch(
            stmt,
            values
                .into_iter()
                .map(|post| -> mysql::Params { post.into() }),
        )?;

        Ok(())
    }

    fn insert(self) -> Result<u64, mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::get_stmt(&mut conn)?;

        conn.exec_drop(stmt, self)?;

        let post_id: u64 = conn.exec_first("SELECT LAST_INSERT_ID();", ())?.unwrap();

        Ok(post_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_post_to_params() {
        let mut post = WP_Post::new(1);
        post.post_title = String::from("My Post");

        let params: mysql::Params = post.into();
        match params {
            mysql::Params::Positional(data) => {
                let id = data.first().unwrap();
                assert_eq!(id, &mysql::Value::UInt(0));
                let author_id = &data[1];
                assert_eq!(author_id, &mysql::Value::UInt(1));
                let p_status = &data[7];
                assert_eq!(p_status, &mysql::Value::Bytes("draft".as_bytes().to_vec()));
            }
            _ => panic!("Not positional"),
        }
    }
}
