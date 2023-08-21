use std::{collections::HashMap, fmt::Display};

use crate::sql::{SqlCompareOperator, SqlConditionOperator, SqlOrder, SqlSearchOperators};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Query<T>
where
    T: Display,
{
    pub author: Option<usize>,
    pub author_name: Option<String>,
    pub author__in: Option<Vec<usize>>,
    pub author__not_in: Option<Vec<usize>>,
    pub cat: Option<usize>,
    pub category_name: Option<String>,
    pub category__and: Option<Vec<usize>>,
    pub category__in: Option<Vec<usize>>,
    pub category__not_in: Option<Vec<usize>>,
    pub tag: Option<String>,
    pub tag_id: Option<usize>,
    pub tag__and: Option<Vec<usize>>,
    pub tag__in: Option<Vec<usize>>,
    pub tag__not_in: Option<Vec<usize>>,
    pub tag_slug__and: Option<Vec<String>>,
    pub tag_slug__in: Option<Vec<String>>,
    /**
     * Key is The logical relationship between each inner taxonomy array when there is more than one. Possible values are ‘AND’, ‘OR’. Do not use with a single inner taxonomy array
     */
    pub tax_query: Option<HashMap<String, Vec<TaxonomyRelation<T>>>>,
    pub p: Option<usize>,
    pub name: Option<String>,
    pub page_id: Option<usize>,
    // TODO implement slug getter
    // pub pagename: Option<String>
    pub post_parent: Option<usize>,
    pub post_parent__in: Option<Vec<usize>>,
    pub post_parent__not_in: Option<Vec<usize>>,
    pub post__in: Option<Vec<usize>>,
    pub post__not_in: Option<Vec<usize>>,
    pub post_name__in: Option<usize>,
    // TODO Implement
    // pub post_password: Option<String>,
    /**
     * Retrieves posts by post types, default value is ‘post‘. If ‘tax_query‘ is set for a query, the default value becomes ‘any‘;
     */
    pub post_type: Option<Vec<String>>,
    pub post_status: Option<PostStatus>,
    /**
     * The amount of comments your CPT has to have ( Search operator will do a ‘=’ operation )
     */
    comment_count: Option<usize>,
    pub posts_per_page: Option<usize>,
    pub page: Option<usize>,
    // TODO
    ignore_sticky_posts: Option<bool>,
    pub order: Option<SqlOrder>,
    pub orderby: Option<WpOrderBy>,
    pub year: Option<u32>,
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
    pub m: Option<u32>,
    // TODO
    // pub date_query: ...
    pub meta_key: Option<String>,
    pub meta_value: Option<String>,
    pub meta_value_num: Option<i32>,
    pub meta_compare: Option<SqlSearchOperators>,
    /**
     * HashMap key is relation, value is the meta query
     * relation: The logical relationship between each inner meta_query array when there is more than one.
     * Possible values are ‘AND’, ‘OR’. Do not use with a single inner meta_query array
     */
    pub meta_query: Option<HashMap<String, Vec<MetaQuery>>>,
    post_mime_type: Option<String>,
}

#[derive(Debug)]
pub struct TaxonomyRelation<T>
where
    T: Display,
{
    pub taxonomy: String,
    /**
     * Select taxonomy term by. Possible values are ‘term_id’, ‘name’, ‘slug’ or ‘term_taxonomy_id’. Default value is ‘term_id’.
     */
    pub field: Vec<usize>,
    pub terms: Vec<T>,
    pub include_children: Option<bool>,
    pub operator: Option<SqlConditionOperator>,
    pub comment_count: Option<CommentCount>,
}

#[derive(Debug)]
pub enum PostStatus {
    Publish,
    Pending,
    Draft,
    AutoDraft,
    Future,
    Private,
    Inherit,
    Trash,
    Any,
}

impl PostStatus {
    pub fn val(&self) -> &'static str {
        match self {
            Self::Publish => "publish",
            Self::Pending => "pending",
            Self::Draft => "draft",
            Self::AutoDraft => "auto-draft",
            Self::Future => "future",
            Self::Private => "private",
            Self::Inherit => "inherit",
            Self::Trash => "trash",
            Self::Any => "any",
        }
    }
}

#[derive(Debug)]
pub struct CommentCount {
    /**
     * The amount of comments your post has to have when comparing
     */
    pub value: usize,
    pub compare: SqlCompareOperator,
}

#[derive(Debug)]
pub enum WpOrderBy {
    None,
    ID,
    Author,
    Title,
    Name,
    Type,
    Date,
    Modified,
    Parent,
    CommentCount,
    MetaValue,
    MetaValueNum,
}

impl WpOrderBy {
    pub fn val(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::ID => "ID",
            Self::Author => "post_author",
            Self::Title => "post_title",
            Self::Name => "post_name",
            Self::Type => "post_type",
            Self::Date => "post_date",
            Self::Modified => "post_modified",
            Self::Parent => "post_parent",
            Self::CommentCount => "comment_count",
            Self::MetaValue => "meta_value",
            Self::MetaValueNum => "meta_value",
        }
    }
}

#[derive(Debug)]
pub struct MetaQuery {
    pub key: String,
    pub value: String,
    pub compare: SqlSearchOperators,
    // TODO
    // type
}
