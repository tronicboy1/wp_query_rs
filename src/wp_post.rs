use mysql_common::time::PrimitiveDateTime;
use serde::ser::{Serialize, SerializeStruct};
use std::fmt::Display;

use crate::sql::date::{get_date_now, get_utc_date_now};

pub use self::meta::WpMetaResults;
use self::{meta::WpMeta, post_status::PostStatus};

mod builder;
pub mod meta;
pub mod post_status;
mod sql;

#[cfg(feature = "php")]
mod zval;

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

/// Applies some objects methods to all the fields on a WpPost, where the fields on the
/// WpPost are references.
///
/// # Example
/// ```rust,ignore
/// let mut state = serializer.serialize_struct("WpPost", 23)?;
/// apply_to_all_fields_ref!(state, serialize_field, self);
#[macro_export]
macro_rules! apply_to_all_fields_ref {
    ($obj: ident, $method: ident, $self: expr) => {
        $obj.$method("ID", &$self.ID)?;
        $obj.$method("post_status", &$self.post_status)?;
        $obj.$method("post_author", &$self.post_author)?;
        $obj.$method("post_date", &$self.post_date.to_string())?;
        $obj.$method("post_date_gmt", &$self.post_date_gmt.to_string())?;
        $obj.$method("post_content", &$self.post_content)?;
        $obj.$method("post_title", &$self.post_title)?;
        $obj.$method("post_excerpt", &$self.post_excerpt)?;
        $obj.$method("comment_status", &$self.comment_status)?;
        $obj.$method("ping_status", &$self.ping_status)?;
        $obj.$method("post_password", &$self.post_password)?;
        $obj.$method("post_name", &$self.post_name)?;
        $obj.$method("to_ping", &$self.to_ping)?;
        $obj.$method("pinged", &$self.pinged)?;
        $obj.$method("post_modified", &$self.post_modified.to_string())?;
        $obj.$method("post_modified_gmt", &$self.post_modified_gmt.to_string())?;
        $obj.$method("post_content_filtered", &$self.post_content_filtered)?;
        $obj.$method("post_parent", &$self.post_parent)?;
        $obj.$method("guid", &$self.guid)?;
        $obj.$method("menu_order", &$self.menu_order)?;
        $obj.$method("post_type", &$self.post_type)?;
        $obj.$method("post_mime_type", &$self.post_mime_type)?;
        $obj.$method("comment_count", &$self.comment_count)?;
    };
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
}

impl Serialize for WpPost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("WpPost", 23)?;

        apply_to_all_fields_ref!(state, serialize_field, self);

        state.end()
    }
}

/// Retrieves a post meta field for the given post ID.
pub fn get_post_meta(post_id: u64, meta_key: &str, single: bool) -> WpMetaResults {
    WpMeta::get_post_meta(post_id, meta_key, single).unwrap()
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
