use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn select_category_id() {
    let params = ParamBuilder::new().cat(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn select_category_name() {
    let params = ParamBuilder::new().category_name("uncategorized");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn tag_id() {
    let params = ParamBuilder::new().tag_id(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn category_and_tag() {
    let params = ParamBuilder::new().tag("tag-a").cat(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
}

#[cfg(feature = "query_sync")]
#[test]
fn category_name_and_tag() {
    let params = ParamBuilder::new()
        .tag("tag-a")
        .category_name("uncategorized");

    let posts = WP_Query::new(params).expect("SqlFailed");
}

#[cfg(feature = "query_sync")]
#[test]
fn select_category_in() {
    let params = ParamBuilder::new().category__in(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn select_category_not_in() {
    let params = ParamBuilder::new().category__not_in(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}
