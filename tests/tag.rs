use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn tag_id() {
    let params = ParamBuilder::new().tag_id(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn tag() {
    let params = ParamBuilder::new().tag("tag-a");

    let posts = WP_Query::new(params).expect("SqlFailed");
}

#[cfg(feature = "query_sync")]
#[test]
fn tag_in() {
    let params = ParamBuilder::new().tag__in(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.posts.len() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn tag_not_in() {
    let params = ParamBuilder::new().tag__not_in(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.posts.len() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn tag_slug_and() {
    let params = ParamBuilder::new().tag_slug__and("tag-a");

    let posts = WP_Query::new(params).expect("SqlFailed");
}

#[cfg(feature = "query_sync")]
#[test]
fn tag_slug_in() {
    let params = ParamBuilder::new().tag_slug__in("tag-a");

    let posts = WP_Query::new(params).expect("SqlFailed");
}
