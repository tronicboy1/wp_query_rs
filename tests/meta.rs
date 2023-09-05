use wp_query_rs::*;

#[test]
fn meta_key() {
    let params = ParamBuilder::new().meta_key("my_inserted_meta");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn meta_value() {
    let params = ParamBuilder::new().meta_value("42");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn meta_value_num() {
    let params = ParamBuilder::new().meta_value_num(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn meta_queries() {
    let params = ParamBuilder::new()
        .meta_query(
            MetaQuery {
                key: String::from("my_inserted_meta"),
                value: String::from("42"),
                compare: SqlSearchOperators::Equals,
            },
            MetaRelation::Or,
        )
        .meta_query(
            MetaQuery {
                key: String::from("my_inserted_meta"),
                value: String::from("0"),
                compare: SqlSearchOperators::Equals,
            },
            MetaRelation::Or,
        )
        .meta_query(
            MetaQuery {
                key: String::from("my_inserted_meta"),
                value: String::from("2"),
                compare: SqlSearchOperators::Like,
            },
            MetaRelation::Or,
        )
        .meta_query(
            MetaQuery {
                key: String::from("my_custom_rs_bulk_meta"),
                value: String::from("1"),
                compare: SqlSearchOperators::GreaterThan,
            },
            MetaRelation::Or,
        )
        .meta_query(
            MetaQuery {
                key: String::from("my_custom_rs_bulk_meta"),
                value: String::from("2"),
                compare: SqlSearchOperators::LessThanOrEqualTo,
            },
            MetaRelation::And,
        )
        .meta_query(
            MetaQuery {
                key: String::from("my_custom_rs_bulk_meta"),
                value: String::from("3"),
                compare: SqlSearchOperators::NotLike,
            },
            MetaRelation::And,
        )
        .meta_query(
            MetaQuery {
                key: String::from("my_inserted_meta"),
                value: String::from("2"),
                compare: SqlSearchOperators::NotEquals,
            },
            MetaRelation::And,
        );

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}
