use mysql::{prelude::Queryable, PooledConn, Statement};
use mysql_common::prelude::ToValue;

use crate::sql::get_conn;

use super::WP_Post;

impl WP_Post {
    fn get_stmt(conn: &mut PooledConn) -> Result<Statement, mysql::Error> {
        conn.prep(
            "INSERT INTO `wp_posts` (
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
        let mut conn = get_conn()?;

        let stmt = Self::get_stmt(&mut conn)?;

        conn.exec_drop(stmt, self)?;

        let post_id: u64 = conn.exec_first("SELECT LAST_INSERT_ID();", ())?.unwrap();

        Ok(post_id)
    }

    pub fn insert_bulk(v: Vec<Self>) -> Result<(), mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::get_stmt(&mut conn)?;

        conn.exec_batch(
            stmt,
            v.into_iter().map(|post| -> mysql::Params { post.into() }),
        )?;

        Ok(())
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
