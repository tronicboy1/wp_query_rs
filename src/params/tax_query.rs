use std::collections::HashMap;

use crate::sql::SqlConditionOperator;

#[derive(Debug)]
pub struct TaxQuery<'a> {
    pub taxonomy: &'a str,
    /**
     * Select taxonomy term by. Possible values are ‘term_id’, ‘name’, ‘slug’ or ‘term_taxonomy_id’. Default value is ‘term_id’.
     */
    pub field: TaxField,
    pub terms: Vec<String>,
    pub include_children: bool,
    pub operator: SqlConditionOperator,
}

impl<'a> TaxQuery<'a> {
    pub fn new_single_tax_map(r: Self) -> HashMap<TaxRelation, Vec<Self>> {
        let k = Self::get_single_tax_key();
        let mut m = HashMap::new();

        m.insert(k, vec![r]);

        m
    }

    fn get_single_tax_key() -> TaxRelation {
        TaxRelation::Single
    }

    pub fn new<T>(taxonomy: &'a str, terms: Vec<T>) -> Self
    where
        T: std::fmt::Display,
    {
        Self {
            taxonomy,
            field: TaxField::TermId,
            terms: terms.into_iter().map(|v| v.to_string()).collect(),
            include_children: true,
            operator: SqlConditionOperator::In,
        }
    }

    pub fn add_term<T>(mut self, term: T) -> Self
    where
        T: std::fmt::Display,
    {
        self.terms.push(term.to_string());

        self
    }

    pub fn field(mut self, field: TaxField) -> Self {
        self.field = field;

        self
    }

    pub fn operator(mut self, operator: SqlConditionOperator) -> Self {
        self.operator = operator;

        self
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TaxRelation {
    And,
    Or,
    Single,
}

impl std::fmt::Display for TaxRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And | Self::Single => "AND",
                Self::Or => "OR",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaxField {
    TermId,
    Name,
    Slug,
    TermTaxonomyId,
}

impl std::fmt::Display for TaxField {
    /// requires the addition of the wp table prefix
    ///
    /// # Example
    /// ```
    /// use wp_query_rs::*;
    ///
    /// let field = TaxField::TermTaxonomyId;
    /// format!("wp_{}", field);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Name => "terms.name",
                Self::Slug => "terms.slug",
                Self::TermId => "terms.term_id",
                Self::TermTaxonomyId => "term_taxonomy.term_taxonomy_id",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let terms = vec![String::from("1")];
        let tax = TaxQuery::new("category", terms.clone());

        assert_eq!(tax.terms, terms);
        assert_eq!(tax.taxonomy, "category");
        assert_eq!(tax.operator, SqlConditionOperator::In);
        assert_eq!(tax.field, TaxField::TermId);
    }

    #[test]
    fn can_add_term() {
        let tax = TaxQuery::new("category", vec![String::from("1")]).add_term("Hello");
        assert_eq!(tax.terms, vec![String::from("1"), String::from("Hello")]);
    }

    #[test]
    fn can_change_field() {
        let tax = TaxQuery::new("category", vec![String::from("1")]).field(TaxField::Name);
        assert_eq!(tax.field, TaxField::Name);
    }

    #[test]
    fn can_change_operator() {
        let tax = TaxQuery::new("category", vec![String::from("1")])
            .operator(SqlConditionOperator::NotIn);
        assert_eq!(tax.operator, SqlConditionOperator::NotIn);
    }
}
