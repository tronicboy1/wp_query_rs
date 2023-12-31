use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_id() {
    let params = ParamBuilder::new().s("a");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
    assert!(posts.posts.iter().all(|p| p.post_content.contains("a")));
}
