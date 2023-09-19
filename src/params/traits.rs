use std::fmt::Display;

use crate::{MetaQuery, MetaRelation, PostStatus, SqlSearchOperators};

use super::post_type::PostType;

#[allow(non_snake_case)]
pub trait PostQueryable<'a> {
    /// use page id to return only child pages. Set to 0 to return only top-level entries.
    fn post_parent(self, id: u64) -> Self;

    /// use post ids. Specify posts whose parent is in an array
    fn post_parent__in(self, id: u64) -> Self;

    /// use post ids. Specify posts whose parent is not in an array
    fn post_parent__not_in(self, id: u64) -> Self;

    /// use post ids. Specify posts to retrieve.
    fn post__in(self, id: u64) -> Self;

    /// use post ids. Specify post NOT to retrieve.
    fn post__not_in(self, id: u64) -> Self;

    fn post_name__in(self, s: &'a str) -> Self;

    /// use post types. Retrieves posts by post types, default value is ‘post‘.
    fn post_type(self, post_type: PostType<'a>) -> Self;

    /// Queries all post types. Will be overwritten if there is another call to post_type after this.
    fn post_type_all(self) -> Self;

    fn post_status(self, status: PostStatus) -> Self;
}

pub trait MetaQueryable<'a> {
    /// Custom field key.
    fn meta_key(self, key: &'a str) -> Self;

    /// Custom field value.
    fn meta_value(self, val: impl Display) -> Self;

    /// Custom field value (number).
    fn meta_value_num(self, n: i64) -> Self;

    /// Operator to test the ‘meta_value‘
    fn meta_compare(self, compare: SqlSearchOperators) -> Self;

    fn meta_query(self, query: MetaQuery, relation: MetaRelation) -> Self;
}
