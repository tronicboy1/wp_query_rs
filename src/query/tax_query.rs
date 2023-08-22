use std::collections::HashMap;

use crate::sql::SqlConditionOperator;

#[derive(Debug)]
pub struct TaxQuery {
    pub taxonomy: String,
    /**
     * Select taxonomy term by. Possible values are ‘term_id’, ‘name’, ‘slug’ or ‘term_taxonomy_id’. Default value is ‘term_id’.
     */
    pub field: TaxField,
    pub terms: Vec<String>,
    pub include_children: bool,
    pub operator: SqlConditionOperator,
}

impl TaxQuery {
    pub fn new_single_tax_map(r: Self) -> HashMap<TaxRelation, Vec<Self>> {
        let k = Self::get_single_tax_key();
        let mut m = HashMap::new();

        m.insert(k, vec![r]);

        m
    }

    fn get_single_tax_key() -> TaxRelation {
        TaxRelation::Single
    }

    pub fn new(taxonomy: String, terms: Vec<String>) -> Self {
        Self {
            taxonomy,
            field: TaxField::TermId,
            terms,
            include_children: true,
            operator: SqlConditionOperator::In,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TaxRelation {
    And,
    Or,
    Single,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaxField {
    TermId,
    Name,
    Slug,
    TermTaxonomyId,
}

#[cfg(test)]
mod tests {
    use crate::{query::tax_query::TaxField, sql::SqlConditionOperator};

    use super::TaxQuery;

    #[test]
    fn can_create() {
        let tax_name = String::from("category");
        let terms = vec![String::from("1")];
        let tax = TaxQuery::new(tax_name.clone(), terms.clone());
        assert_eq!(tax.terms, terms);
        assert_eq!(tax.taxonomy, tax_name);
        assert_eq!(tax.operator, SqlConditionOperator::In);
        assert_eq!(tax.field, TaxField::TermId);
    }
}
