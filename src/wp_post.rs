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
