use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn all_posts_are_publish() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Publish);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Publish));
}

#[cfg(feature = "query_sync")]
#[test]
fn all_posts_are_draft() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Draft);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Draft));
}

#[cfg(feature = "query_sync")]
#[test]
fn all_posts_are_auto_draft() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::AutoDraft);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::AutoDraft));
}

#[cfg(feature = "query_sync")]
#[test]
fn all_posts_are_private() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Private);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Private));
}

#[cfg(feature = "query_sync")]
#[test]
fn all_posts_are_trash() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Trash);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Trash));
}

#[cfg(feature = "query_sync")]
#[test]
fn all_posts_are_future() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Future);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Future));
}

#[cfg(feature = "query_sync")]
#[test]
fn post_type_any() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Any);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(!posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Publish));
}
