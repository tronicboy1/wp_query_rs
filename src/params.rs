pub mod date_query;
pub mod meta_query;
pub mod orderby;
pub mod param_builder;
pub mod post_type;
pub mod tax_query;
pub mod traits;

#[cfg(feature = "php")]
mod zval;

use std::collections::HashMap;

use crate::sql::{SqlCompareOperator, SqlOrder, SqlSearchOperators};
use crate::wp_post::post_status::PostStatus;

use self::date_query::DateQuery;
use self::meta_query::{MetaQuery, MetaRelation};
use self::orderby::WpOrderBy;
use self::post_type::PostType;
use self::tax_query::{TaxQuery, TaxRelation};

/// Configuration for running a WordPress database query.
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Params<'a> {
    pub author: Option<u64>,
    pub author_name: Option<&'a str>,
    pub author__in: Option<Vec<u64>>,
    pub author__not_in: Option<Vec<u64>>,
    pub term_and: Option<Vec<u64>>,
    pub term_in: Option<Vec<u64>>,
    pub term_not_in: Option<Vec<u64>>,
    pub term_slug_and: Option<Vec<&'a str>>,
    pub term_slug_in: Option<Vec<&'a str>>,
    /**
     * Key is The logical relationship between each inner taxonomy array when there is more than one. Possible values are ‘AND’, ‘OR’. Do not use with a single inner taxonomy array
     */
    pub tax_query: Option<HashMap<TaxRelation, Vec<TaxQuery<'a>>>>,
    pub s: Option<&'a str>,
    pub p: Option<u64>,
    pub name: Option<&'a str>,
    pub page_id: Option<u64>,
    pub pagename: Option<&'a str>,
    pub post_parent: Option<u64>,
    pub post_parent__in: Option<Vec<u64>>,
    pub post_parent__not_in: Option<Vec<u64>>,
    pub post__in: Option<Vec<u64>>,
    pub post__not_in: Option<Vec<u64>>,
    pub post_name__in: Option<Vec<&'a str>>,
    pub post_password: Option<&'a str>,
    /**
     * Retrieves posts by post types, default value is ‘post‘. If ‘tax_query‘ is set for a query, the default value becomes ‘any‘;
     */
    pub post_type: Option<Vec<PostType<'a>>>,
    pub post_status: Option<PostStatus>,
    /**
     * The amount of comments your CPT has to have ( Search operator will do a ‘=’ operation )
     */
    pub comment_count: Option<u64>,
    pub posts_per_page: Option<u64>,
    pub page: Option<u64>,
    pub ignore_sticky_posts: Option<bool>,
    pub order: Option<SqlOrder>,
    pub orderby: Option<WpOrderBy>,
    pub year: Option<u16>,
    pub monthnum: Option<u8>,
    /**
     * Week of the year (from 0 to 53). Uses MySQL WEEK command. The mode is dependent on the “start_of_week” option.
     */
    pub w: Option<u8>,
    pub day: Option<u8>,
    pub hour: Option<u8>,
    pub minute: Option<u8>,
    pub second: Option<u8>,
    /**
     * YearMonth (For e.g.: 201307).
     */
    pub m: Option<u64>,
    pub date_query: Option<Vec<DateQuery>>,
    pub meta_key: Option<&'a str>,
    pub meta_value: Option<String>,
    pub meta_value_num: Option<i64>,
    pub meta_compare: Option<SqlSearchOperators>,
    /**
     * HashMap key is relation, value is the meta query
     * relation: The logical relationship between each inner meta_query array when there is more than one.
     * Possible values are ‘AND’, ‘OR’. Do not use with a single inner meta_query array
     */
    pub meta_query: Option<HashMap<MetaRelation, Vec<MetaQuery>>>,
    pub post_mime_type: Option<&'a str>,
}

impl<'a> Params<'a> {
    pub fn new() -> Self {
        Self {
            author: None,
            author_name: None,
            author__in: None,
            author__not_in: None,
            // Holds ids for both categories and tags
            term_and: None,
            term_in: None,
            term_not_in: None,
            term_slug_and: None,
            term_slug_in: None,
            tax_query: None,
            s: None,
            p: None,
            name: None,
            page_id: None,
            pagename: None,
            post_parent: None,
            post_parent__in: None,
            post_parent__not_in: None,
            post__in: None,
            post__not_in: None,
            post_name__in: None,
            post_password: None,
            post_type: None,
            post_status: None,
            comment_count: None,
            posts_per_page: None,
            page: None,
            ignore_sticky_posts: None,
            order: None,
            orderby: None,
            year: None,
            monthnum: None,
            w: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            m: None,
            date_query: None,
            meta_key: None,
            meta_value: None,
            meta_value_num: None,
            meta_compare: None,
            meta_query: None,
            post_mime_type: None,
        }
    }
}

#[derive(Debug)]
pub struct CommentCount {
    /**
     * The amount of comments your post has to have when comparing
     */
    pub value: u64,
    pub compare: SqlCompareOperator,
}

#[cfg(test)]
mod tests {}
