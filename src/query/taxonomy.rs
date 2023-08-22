use std::{collections::HashMap, fmt::Display};

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

impl<T> TaxonomyRelation<T>
where
    T: Display,
{
    pub fn new_single_tax_map(r: Self) -> HashMap<String, Vec<Self>> {
        let k = Self::get_single_tax_key();
        let mut m = HashMap::new();

        m.insert(k, vec![r]);

        m
    }

    fn get_single_tax_key() -> String {
        String::from("SINGLE")
    }
}

#[cfg(test)]
mod tests {}
