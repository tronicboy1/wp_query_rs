pub mod fields;
pub mod hierarchical;
pub mod orderby;

use std::{collections::HashMap, ops::DerefMut};

use crate::{
    sql::{cast_type::CastType, SqlCompareOperator},
    MetaQuery, MetaRelation, PostStatus, SqlOrder,
};

use self::{fields::Fields, hierarchical::Hierarchy, orderby::Orderby};

use super::{comment_approved::CommentApproved, comment_type::CommentType};

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct WpCommentQueryArgs {
    pub author_email: Option<String>,
    pub author_url: Option<String>,
    pub author__in: Option<Vec<u64>>,
    pub author__not_in: Option<Vec<u64>>,
    pub comment__in: Option<Vec<u64>>,
    pub comment__not_in: Option<Vec<u64>>,
    pub count: bool,
    // TODO
    // pub date_query: WpDateQuery,
    pub fields: Fields,
    pub include_unapproved: Option<Vec<u64>>,
    pub karma: Option<u64>,
    pub meta_key: Option<Vec<String>>,
    pub meta_value: Option<Vec<String>>,
    pub meta_compare: SqlCompareOperator,
    pub meta_type: CastType,
    /**
     * HashMap key is relation, value is the meta query
     * relation: The logical relationship between each inner meta_query array when there is more than one.
     * Possible values are ‘AND’, ‘OR’. Do not use with a single inner meta_query array
     */
    pub meta_query: Option<HashMap<MetaRelation, Vec<MetaQuery>>>,
    pub number: Option<u64>,
    pub paged: Option<u64>,
    pub offset: Option<u64>,
    pub orderby: Orderby,
    pub order: SqlOrder,
    pub parent: Option<u64>,
    pub parent__in: Option<Vec<u64>>,
    pub parent__not_in: Option<Vec<u64>>,
    pub post_author__in: Option<Vec<u64>>,
    pub post_author__not_in: Option<Vec<u64>>,
    pub post_id: Option<u64>,
    pub post__in: Option<Vec<u64>>,
    pub post__not_in: Option<Vec<u64>>,
    pub post_author: Option<u64>,
    pub post_status: Option<PostStatus>,
    pub post_type: Option<Vec<String>>,
    pub post_name: Option<String>,
    pub post_parent: Option<u64>,
    pub search: Option<String>,
    pub status: Option<CommentApproved>,
    pub comment_type: Option<CommentType>,
    pub comment_type__in: Option<Vec<CommentType>>,
    pub comment_type__not_in: Option<Vec<CommentType>>,
    pub user_id: Option<u64>,
    pub hierarchical: Option<Hierarchy>,
}

pub struct WpCommentArgBuilder {
    args: WpCommentQueryArgs,
}

impl WpCommentArgBuilder {
    pub fn new() -> Self {
        Self {
            args: WpCommentQueryArgs {
                author_email: None,
                author_url: None,
                author__in: None,
                author__not_in: None,
                comment__in: None,
                comment__not_in: None,
                count: false,
                fields: Fields::All,
                include_unapproved: None,
                karma: None,
                meta_key: None,
                meta_value: None,
                meta_compare: SqlCompareOperator::Equals,
                meta_type: CastType::Char,
                meta_query: None,
                number: None,
                paged: None,
                offset: None,
                orderby: Orderby::DateGmt,
                order: SqlOrder::Desc,
                parent: None,
                parent__in: None,
                parent__not_in: None,
                post_author__in: None,
                post_author__not_in: None,
                post_id: None,
                post__in: None,
                post__not_in: None,
                post_author: None,
                post_status: None,
                post_type: None,
                post_name: None,
                post_parent: None,
                search: None,
                status: None,
                comment_type: None,
                comment_type__in: None,
                comment_type__not_in: None,
                user_id: None,
                hierarchical: None,
            },
        }
    }
}

impl std::ops::Deref for WpCommentArgBuilder {
    type Target = WpCommentQueryArgs;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl DerefMut for WpCommentArgBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_author_email() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_author_url() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_author_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_author_not_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_comment_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_comment_not_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_count() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_date_query() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_fields() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_include_unapproved() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_karma() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_meta_key() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_meta_value() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_meta_compare() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_meta_compare_key() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_author_meta_type() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_meta_type_key() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_meta_query() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_number() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_paged() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_offset() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_orderby() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_order() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_parent() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_parent_in() {
        WpCommentArgBuilder::new();
    }
    #[test]
    fn can_add_parent_not_in() {
        WpCommentArgBuilder::new();
    }
    #[test]
    fn can_add_post_author_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_post_author_not_in() {
        WpCommentArgBuilder::new();
    }
    #[test]
    fn can_add_post_id() {
        WpCommentArgBuilder::new();
    }
    #[test]
    fn can_add_post_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_post_not_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_post_author() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_post_status() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_post_type() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_post_name() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_post_parent() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_search() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_status() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_type() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_type_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_type_not_in() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_user_id() {
        WpCommentArgBuilder::new();
    }

    #[test]
    fn can_add_hierarchical() {
        WpCommentArgBuilder::new();
    }
}
