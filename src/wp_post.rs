use ext_php_rs::{
    convert::IntoZval,
    flags::DataType,
    types::{ZendObject, Zval},
};
use mysql_common::time::Date;

use self::post_status::PostStatus;

pub mod post_status;

#[derive(Debug)]
#[allow(non_camel_case_types, non_snake_case)]
pub struct WP_Post {
    pub ID: u64,
    pub post_author: u64,
    pub post_date: Date,
    pub post_date_gmt: Date,
    pub post_content: String,
    pub post_title: String,
    pub post_excerpt: String,
    pub post_status: PostStatus,
    pub comment_status: String,
    pub ping_status: String,
    pub post_password: String,
    pub post_name: String,
    pub to_ping: String,
    pub pinged: String,
    pub post_modified: Date,
    pub post_modified_gmt: Date,
    pub post_content_filtered: String,
    pub post_parent: u64,
    pub guid: String,
    pub menu_order: u64,
    pub post_type: String,
    pub post_mime_type: String,
    pub comment_count: u64,
}

impl IntoZval for WP_Post {
    const TYPE: ext_php_rs::flags::DataType = DataType::Object(Some("WP_Post_Rs"));

    fn into_zval(self, persistent: bool) -> ext_php_rs::error::Result<ext_php_rs::types::Zval> {
        let mut zobj = ZendObject::new_stdclass();

        zobj.set_property("ID", self.ID)?;
        zobj.set_property("post_status", self.post_status)?;

        zobj.into_zval(persistent)
    }

    fn set_zval(self, zv: &mut Zval, persistent: bool) -> ext_php_rs::error::Result<()> {
        let mut zobj = ZendObject::new_stdclass();

        zobj.set_property("ID", self.ID)?;
        zobj.set_property("post_status", self.post_status)?;

        zv.set_object(&mut zobj);

        Ok(())
    }
}
