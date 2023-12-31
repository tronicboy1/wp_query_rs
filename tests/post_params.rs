use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_id() {
    let params = ParamBuilder::new().p(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 1);
    assert_eq!(posts.posts.first().unwrap().ID, 1);
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_type() {
    let params = ParamBuilder::new().post_type(PostType::Page);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_multiple_post_types() {
    let params = ParamBuilder::new()
        .post_type(PostType::Page)
        .post_type(PostType::Post);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_name() {
    let params = ParamBuilder::new().name("");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_parent() {
    let params = ParamBuilder::new().post_parent(0);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
    assert!(posts.posts.iter().all(|p| p.post_parent == 0));
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_parent_in() {
    let params = ParamBuilder::new().post_parent__in(0).post__in(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_parent == 0 || p.post_parent == 1));
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_parent_not_in() {
    let params = ParamBuilder::new().post_parent__not_in(0);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.posts.iter().all(|p| p.post_parent != 0));
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_id_in() {
    let params = ParamBuilder::new().post_type_all().post__in(1).post__in(2);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 2);
    assert_eq!(posts.posts.first().unwrap().ID, 1);
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_id_not_in() {
    let params = ParamBuilder::new().post__not_in(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.posts.iter().all(|p| p.ID != 1));
}

#[cfg(feature = "query_sync")]
#[test]
fn select_by_post_name_in() {
    let params = ParamBuilder::new().post_name__in("");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 0);
}
