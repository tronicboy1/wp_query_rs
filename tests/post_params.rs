use wp_query_rs::*;

#[test]
fn select_by_post_id() {
    let params = ParamBuilder::new().p(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert_eq!(posts.post_count(), 1);
    assert_eq!(posts.posts.first().unwrap().ID, 1);
}
