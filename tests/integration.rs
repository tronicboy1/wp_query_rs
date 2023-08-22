use wp_query_rs::*;

#[test]
fn default_selects_posts() {
    let params = ParamBuilder::new();

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[test]
fn tag() {
    let params = ParamBuilder::new().tag_id(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[test]
fn meta_data() {
    let params = ParamBuilder::new().meta_value(String::from("1"));

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[test]
fn paginate() {
    let params = ParamBuilder::new().page(2);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);

    let params = ParamBuilder::new().page(32);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}
