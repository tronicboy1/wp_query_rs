use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn orderby_post_date() {
    let params = ParamBuilder::new()
        .orderby(WpOrderBy::Date)
        .order(SqlOrder::Desc);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn orderby_author() {
    let params = ParamBuilder::new()
        .orderby(WpOrderBy::Author)
        .order(SqlOrder::Desc);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn orderby_comments() {
    let params = ParamBuilder::new()
        .orderby(WpOrderBy::CommentCount)
        .order(SqlOrder::Asc);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn orderby_modified() {
    let params = ParamBuilder::new()
        .orderby(WpOrderBy::Modified)
        .order(SqlOrder::Asc);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn orderby_title() {
    let params = ParamBuilder::new()
        .orderby(WpOrderBy::Title)
        .order(SqlOrder::Asc);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}
