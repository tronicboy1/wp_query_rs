use self::post_status::PostStatus;

pub mod post_status;

#[allow(non_camel_case_types, non_snake_case)]
pub struct WP_Post {
    pub ID: u32,
    pub post_author: u32,
    pub post_date: String,
    pub post_date_gmt: String,
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
    pub post_modified: String,
    pub post_modified_gmt: String,
    pub post_content_filtered: String,
    pub post_parent: u32,
    pub guid: String,
    pub menu_order: u32,
    pub post_type: String,
    pub post_mime_type: String,
    pub comment_count: u32,
}
