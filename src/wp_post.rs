use ext_php_rs::{
    boxed::ZBox,
    convert::{FromZval, IntoZval},
    ffi::_zend_object,
    flags::DataType,
    types::{ZendObject, Zval},
};
use mysql_common::time::PrimitiveDateTime;
use std::fmt::Display;

use crate::sql::date::{get_date_now, get_utc_date_now};

pub use self::meta::WpMetaResults;
use self::{meta::WpMeta, post_status::PostStatus};

mod builder;
pub mod meta;
pub mod post_status;
mod sql;

/// A representation of a wp_posts entity queried from the database
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct WpPost {
    pub ID: u64,
    pub post_author: u64,
    pub post_date: PrimitiveDateTime,
    pub post_date_gmt: PrimitiveDateTime,
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
    pub post_modified: PrimitiveDateTime,
    pub post_modified_gmt: PrimitiveDateTime,
    pub post_content_filtered: String,
    pub post_parent: u64,
    pub guid: String,
    pub menu_order: u64,
    pub post_type: String,
    pub post_mime_type: String,
    pub comment_count: u64,
}

impl WpPost {
    pub fn new(post_author: u64) -> Self {
        let now = get_date_now();
        let now_utc = get_utc_date_now();

        Self {
            ID: 0,
            post_author,
            post_date: now.clone(),
            post_date_gmt: now_utc.clone(),
            post_content: String::new(),
            post_title: String::new(),
            post_excerpt: String::new(),
            post_status: PostStatus::Draft,
            comment_status: String::new(),
            ping_status: String::new(),
            post_password: String::new(),
            post_name: String::new(),
            to_ping: String::new(),
            pinged: String::new(),
            post_modified: now.clone(),
            post_modified_gmt: now_utc.clone(),
            post_content_filtered: String::new(),
            post_parent: 0,
            guid: String::new(),
            menu_order: 0,
            post_type: String::from("post"),
            post_mime_type: String::new(),
            comment_count: 0,
        }
    }

    fn build_zobj(self) -> ext_php_rs::error::Result<ZBox<_zend_object>> {
        let mut zobj = ZendObject::new_stdclass();

        zobj.set_property("ID", self.ID)?;
        zobj.set_property("post_status", self.post_status)?;
        zobj.set_property("post_author", self.post_author)?;
        zobj.set_property("post_date", self.post_date.to_string())?;
        zobj.set_property("post_date_gmt", self.post_date_gmt.to_string())?;
        zobj.set_property("post_content", self.post_content)?;
        zobj.set_property("post_title", self.post_title)?;
        zobj.set_property("post_excerpt", self.post_excerpt)?;
        zobj.set_property("comment_status", self.comment_status)?;
        zobj.set_property("ping_status", self.ping_status)?;
        zobj.set_property("post_password", self.post_password)?;
        zobj.set_property("post_name", self.post_name)?;
        zobj.set_property("to_ping", self.to_ping)?;
        zobj.set_property("pinged", self.pinged)?;
        zobj.set_property("post_modified", self.post_modified.to_string())?;
        zobj.set_property("post_modified_gmt", self.post_modified_gmt.to_string())?;
        zobj.set_property("post_content_filtered", self.post_content_filtered)?;
        zobj.set_property("post_parent", self.post_parent)?;
        zobj.set_property("guid", self.guid)?;
        zobj.set_property("menu_order", self.menu_order)?;
        zobj.set_property("post_type", self.post_type)?;
        zobj.set_property("post_mime_type", self.post_mime_type)?;
        zobj.set_property("comment_count", self.comment_count)?;

        Ok(zobj)
    }
}

impl IntoZval for WpPost {
    const TYPE: ext_php_rs::flags::DataType = DataType::Object(Some("RS_WP_Post"));

    fn into_zval(self, persistent: bool) -> ext_php_rs::error::Result<ext_php_rs::types::Zval> {
        let zobj = self.build_zobj()?;

        zobj.into_zval(persistent)
    }

    fn set_zval(self, zv: &mut Zval, _persistent: bool) -> ext_php_rs::error::Result<()> {
        let mut zobj = self.build_zobj()?;

        zv.set_object(&mut zobj);

        Ok(())
    }
}

impl<'a> FromZval<'a> for WpPost {
    const TYPE: ext_php_rs::flags::DataType = DataType::Object(Some("RS_WP_Post"));

    // Do not implement as not used, must satisfy ext-php-rs traits
    fn from_zval(zval: &'a Zval) -> Option<Self> {
        if let Some(_array) = zval.array() {}

        if let Some(_obj) = zval.object() {}

        None
    }
}

/// Retrieves a post meta field for the given post ID.
pub fn get_post_meta(post_id: u64, meta_key: &str, single: bool) -> WpMetaResults {
    WpMeta::get_post_meta(post_id, meta_key, single)
}

/// Adds a meta field to the given post.
pub fn add_post_meta(
    post_id: u64,
    meta_key: &str,
    meta_value: impl Display,
) -> Result<u64, mysql::Error> {
    WpMeta::add_post_meta(post_id, meta_key, meta_value)
}

#[cfg(test)]
mod tests {}
