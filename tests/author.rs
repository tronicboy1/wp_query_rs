use wp_query_rs::*;

#[test]
fn can_join_user_table() {
    let params = ParamBuilder::new().author_name("");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() == 0);
}

#[test]
fn can_search_author_name() {
    let params = ParamBuilder::new().author_name("admin");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[test]
fn query_user_id() {
    let params = ParamBuilder::new().author(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[test]
fn query_user_id_in() {
    let params = ParamBuilder::new().author__in(1).author__in(2);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.posts.iter().all(|p| p.post_author == 1 || p.post_author == 2));
}

#[test]
fn query_user_id_not_in() {
    let params = ParamBuilder::new().author__not_in(1).author__not_in(2);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.posts.iter().all(|p| p.post_author != 1 && p.post_author != 2));
}

#[test]
fn query_user_id_not_in_and_in() {
    let params = ParamBuilder::new().author__not_in(1).author__in(2);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.posts.iter().all(|p| p.post_author != 1 && p.post_author == 2));
}
