#[cfg(feature = "query_sync")]
use mysql::prelude::*;
#[cfg(feature = "query_async")]
use mysql_async::prelude::*;

use crate::sql::find_col;
#[cfg(any(feature = "query_sync", feature = "query_async"))]
use crate::sql::{get_conn, traits::Insertable};

use super::{get_date_now, get_utc_date_now, WpPost};

macro_rules! get_stmt {
    ($conn: ident) => {
        $conn.prep(
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
    };
}

impl WpPost {
    #[cfg(feature = "query_sync")]
    fn get_stmt(conn: &mut impl Queryable) -> Result<mysql::Statement, mysql::Error> {
        get_stmt!(conn)
    }
    #[cfg(feature = "query_async")]
    async fn get_stmt(
        conn: &mut mysql_async::Conn,
    ) -> Result<mysql_async::Statement, mysql_async::Error> {
        get_stmt!(conn).await
    }

    #[cfg(feature = "query_sync")]
    pub fn insert(self) -> Result<u64, mysql::Error> {
        <Self as Insertable>::insert(self)
    }

    #[cfg(feature = "query_sync")]
    pub fn insert_bulk(v: Vec<Self>) -> Result<(), mysql::Error> {
        <Self as Insertable>::batch(v)
    }
}

#[cfg(any(feature = "query_sync", feature = "query_async"))]
impl Into<mysql_common::params::Params> for WpPost {
    fn into(self) -> mysql_common::params::Params {
        mysql_common::params::Params::Positional(vec![
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

#[macro_export]
macro_rules! ok_or_row_error {
    ($row: ident, $col: expr) => {
        find_col(&mut $row, $col).ok_or_else(|| mysql_common::FromRowError($row.clone()))?
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

#[cfg(any(feature = "query_sync", feature = "query_async"))]
impl Insertable for WpPost {
    #[cfg(feature = "query_sync")]
    fn batch(values: impl IntoIterator<Item = Self>) -> Result<(), mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::get_stmt(&mut conn)?;

        conn.exec_batch(
            stmt,
            values
                .into_iter()
                .map(|post| -> mysql_common::params::Params { post.into() }),
        )?;

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

            let stmt = Self::get_stmt(&mut conn).await?;

            conn.exec_batch(
                stmt,
                values
                    .into_iter()
                    .map(|post| -> mysql_common::params::Params { post.into() }),
            )
            .await?;

            Ok(())
        };

        Box::pin(fut)
    }

    #[cfg(feature = "query_sync")]
    fn insert(self) -> Result<u64, mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = Self::get_stmt(&mut conn)?;

        conn.exec_drop(stmt, self)?;

        let post_id: u64 = conn.exec_first("SELECT LAST_INSERT_ID();", ())?.unwrap();

        Ok(post_id)
    }

    #[cfg(feature = "query_async")]
    fn insert(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<u64, mysql_async::Error>>>> {
        let fut = async {
            let mut conn = get_conn().await?;

            let stmt = Self::get_stmt(&mut conn).await?;

            conn.exec_drop(stmt, self).await?;

            let post_id: u64 = conn
                .exec_first("SELECT LAST_INSERT_ID();", ())
                .await?
                .unwrap();

            Ok(post_id)
        };

        Box::pin(fut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(any(feature = "query_sync", feature = "query_async"))]
    fn can_convert_post_to_params() {
        let mut post = WpPost::new(1);
        post.post_title = String::from("My Post");

        let params: mysql_common::params::Params = post.into();
        match params {
            mysql_common::params::Params::Positional(data) => {
                let id = data.first().unwrap();
                assert_eq!(id, &mysql_common::Value::UInt(0));
                let author_id = &data[1];
                assert_eq!(author_id, &mysql_common::Value::UInt(1));
                let p_status = &data[7];
                assert_eq!(
                    p_status,
                    &mysql_common::Value::Bytes("draft".as_bytes().to_vec())
                );
            }
            _ => panic!("Not positional"),
        }
    }
}
