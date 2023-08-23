use wp_query_rs::*;

#[test]
fn tag_id() {
    let params = ParamBuilder::new().tag_id(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[test]
fn tag() {
    let params = ParamBuilder::new().tag(String::from("tag-a"));

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
}
