use wp_query_rs::*;

#[test]
fn can_search_single() {
    let params = ParamBuilder::new()
        .tax_query(
            TaxQuery::new("category", vec!["uncategorized"]).field(TaxField::Name),
            None,
        )
        .order(SqlOrder::Desc);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn can_search_multiple() {
    let params = ParamBuilder::new()
        .tax_query(
            TaxQuery::new("category", vec!["uncategorized"]).field(TaxField::Name),
            Some(TaxRelation::And),
        )
        .tax_query(
            TaxQuery::new("category", vec![1]).field(TaxField::TermId),
            Some(TaxRelation::And),
        )
        .order(SqlOrder::Desc);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn can_search_by_tax_field() {
    let params = ParamBuilder::new()
        .tax_query(
            TaxQuery::new("category", vec![0]).field(TaxField::TermTaxonomyId),
            Some(TaxRelation::And),
        )
        .order(SqlOrder::Desc);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() == 0);
}
