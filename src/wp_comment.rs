mod arg_builder;
mod comment_approved;
mod comment_type;

use std::net;

use mysql_common::time::PrimitiveDateTime;

use crate::sql::date::{get_date_now, get_utc_date_now};

use self::{comment_approved::CommentApproved, comment_type::CommentType};

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct WpComment {
    pub comment_ID: u64,
    pub comment_post_ID: u64,
    pub comment_author: String,
    pub comment_author_email: String,
    pub comment_author_url: String,
    pub comment_author_IP: net::Ipv4Addr,
    pub comment_date: PrimitiveDateTime,
    pub comment_date_gmt: PrimitiveDateTime,
    pub comment_content: String,
    pub comment_karma: u64,
    pub comment_approved: CommentApproved,
    pub comment_agent: String,
    pub comment_type: CommentType,
    pub comment_parent: u64,
    pub user_id: u64,
    children: Option<Vec<WpComment>>,
}

impl WpComment {
    pub fn new(user_id: u64) -> Self {
        Self {
            comment_ID: 0,
            comment_post_ID: 0,
            comment_author: String::new(),
            comment_author_email: String::new(),
            comment_author_url: String::new(),
            comment_author_IP: net::Ipv4Addr::new(0, 0, 0, 0),
            comment_date: get_date_now(),
            comment_date_gmt: get_utc_date_now(),
            comment_content: String::new(),
            comment_karma: 0,
            comment_approved: CommentApproved::Approved,
            comment_agent: String::new(),
            comment_type: CommentType::Comment,
            comment_parent: 0,
            user_id,
            children: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let comment = WpComment::new(1);

        assert_eq!(comment.user_id, 1);
        assert_eq!(comment.comment_approved, CommentApproved::Approved);
    }
}
