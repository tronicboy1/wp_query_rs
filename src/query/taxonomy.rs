use std::fmt::Display;

use crate::sql::SqlConditionOperator;

use super::CommentCount;

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

#[cfg(test)]
mod tests {}
