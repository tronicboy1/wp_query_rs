use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn meta_key() {
    let params = ParamBuilder::new()
        .post_type_all()
        .post_status(PostStatus::Any)
        .meta_key("my_inserted_meta");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn meta_value() {
    let params = ParamBuilder::new().post_type_all().meta_value("42");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn meta_value_num() {
    let params = ParamBuilder::new().post_type_all().meta_value_num(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn meta_queries() {
    let params = ParamBuilder::new()
        .post_type_all()
        .post_status(PostStatus::Any)
        .meta_query(
            MetaQuery::new("my_inserted_meta", "42", SqlSearchOperators::Equals),
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

#[cfg(feature = "query_sync")]
#[test]
fn can_query_exists() {
    let params = ParamBuilder::new()
        .post_type_all()
        .post_status(PostStatus::Any)
        .meta_query(
            MetaQuery::new("my_inserted_meta", "", SqlSearchOperators::Exists),
            MetaRelation::And,
        );

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn can_query_not_exists() {
    let params = ParamBuilder::new()
        .post_type_all()
        .post_status(PostStatus::Any)
        .meta_query(
            MetaQuery::new("my_inserted_meta", "", SqlSearchOperators::NotExists),
            MetaRelation::And,
        );

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}
