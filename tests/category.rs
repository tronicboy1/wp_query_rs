use wp_query_rs::*;

#[test]
fn select_category_id() {
    let params = ParamBuilder::new().cat(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn select_category_name() {
    let params = ParamBuilder::new().category_name(String::from("uncategorized"));

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn tag_id() {
    let params = ParamBuilder::new().tag_id(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[test]
fn category_and_tag() {
    let params = ParamBuilder::new().tag(String::from("tag-a")).cat(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
}

#[test]
fn category_name_and_tag() {
    let params = ParamBuilder::new()
        .tag(String::from("tag-a"))
        .category_name(String::from("uncategorized"));

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
}

#[test]
fn select_category_in() {
    let params = ParamBuilder::new().category__in(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn select_category_not_in() {
    let params = ParamBuilder::new().category__not_in(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}
