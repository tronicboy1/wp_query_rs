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

impl From<mysql::Row> for WpPost {
    fn from(mut row: mysql::Row) -> Self {
        Self {
            ID: find_col(&mut row, "ID").unwrap(),
            post_author: find_col(&mut row, "post_author").unwrap(),
            post_date: find_col(&mut row, "post_date").unwrap_or(get_date_now()),
            post_date_gmt: find_col(&mut row, "post_date_gmt").unwrap_or(get_utc_date_now()),
            post_content: find_col(&mut row, "post_content").unwrap(),
            post_title: find_col(&mut row, "post_title").unwrap(),
            post_excerpt: find_col(&mut row, "post_excerpt").unwrap(),
            post_status: {
                let post_status: String = find_col(&mut row, "post_status").unwrap();
                post_status.into()
            },
            comment_status: find_col(&mut row, "comment_status").unwrap(),
            ping_status: find_col(&mut row, "ping_status").unwrap(),
            post_password: find_col(&mut row, "post_password").unwrap(),
            post_name: find_col(&mut row, "post_name").unwrap(),
            to_ping: find_col(&mut row, "to_ping").unwrap(),
            pinged: find_col(&mut row, "pinged").unwrap(),
            post_modified: find_col(&mut row, "post_modified").unwrap_or(get_date_now()),
            post_modified_gmt: find_col(&mut row, "post_modified_gmt")
                .unwrap_or(get_utc_date_now()),
            post_content_filtered: find_col(&mut row, "post_content_filtered").unwrap(),
            post_parent: find_col(&mut row, "post_parent").unwrap(),
            guid: find_col(&mut row, "guid").unwrap(),
            menu_order: find_col(&mut row, "menu_order").unwrap(),
            post_type: find_col(&mut row, "post_type").unwrap(),
            post_mime_type: find_col(&mut row, "post_mime_type").unwrap(),
            comment_count: find_col(&mut row, "comment_count").unwrap(),
        }
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
