use mysql::{prelude::Queryable, Statement};
use mysql_common::prelude::ToValue;

use crate::{
    sql::{find_col, get_conn},
    Insertable,
};

use super::{get_date_now, get_utc_date_now, WpPost};

impl WpPost {
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

impl Into<mysql::Params> for WpPost {
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

macro_rules! ok_or_row_error {
    ($row: ident, $col: expr) => {
        find_col(&mut $row, $col).ok_or(mysql_common::FromRowError($row.clone()))?
    };
}

impl mysql_common::prelude::FromRow for WpPost {
    fn from_row_opt(mut row: mysql_common::Row) -> Result<Self, mysql_common::FromRowError>
    where
        Self: Sized,
    {
        Ok(Self {
            ID: ok_or_row_error!(row, "ID"),
            post_author: ok_or_row_error!(row, "post_author"),
            post_date: find_col(&mut row, "post_date").unwrap_or(get_date_now()),
            post_date_gmt: find_col(&mut row, "post_date_gmt").unwrap_or(get_utc_date_now()),
            post_content: ok_or_row_error!(row, "post_content"),
            post_title: ok_or_row_error!(row, "post_title"),
            post_excerpt: ok_or_row_error!(row, "post_excerpt"),
            post_status: ok_or_row_error!(row, "post_status"),
            comment_status: ok_or_row_error!(row, "comment_status"),
            ping_status: ok_or_row_error!(row, "ping_status"),
            post_password: ok_or_row_error!(row, "post_password"),
            post_name: ok_or_row_error!(row, "post_name"),
            to_ping: ok_or_row_error!(row, "to_ping"),
            pinged: ok_or_row_error!(row, "pinged"),
            post_modified: find_col(&mut row, "post_modified").unwrap_or(get_date_now()),
            post_modified_gmt: find_col(&mut row, "post_modified_gmt")
                .unwrap_or(get_utc_date_now()),
            post_content_filtered: ok_or_row_error!(row, "post_content_filtered"),
            post_parent: ok_or_row_error!(row, "post_parent"),
            guid: ok_or_row_error!(row, "guid"),
            menu_order: ok_or_row_error!(row, "menu_order"),
            post_type: ok_or_row_error!(row, "post_type"),
            post_mime_type: ok_or_row_error!(row, "post_mime_type"),
            comment_count: ok_or_row_error!(row, "comment_count"),
        })
    }
}

impl Insertable for WpPost {
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
        let mut post = WpPost::new(1);
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
