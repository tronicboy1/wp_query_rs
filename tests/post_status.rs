use wp_query_rs::*;

#[test]
fn all_posts_are_publish() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Publish);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Publish));
}

#[test]
fn all_posts_are_draft() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Draft);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Draft));
}

#[test]
fn all_posts_are_auto_draft() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::AutoDraft);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::AutoDraft));
}

#[test]
fn all_posts_are_private() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Private);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Private));
}

#[test]
fn all_posts_are_trash() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Trash);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Trash));
}

#[test]
fn all_posts_are_future() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Future);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Future));
}

#[test]
fn post_type_any() {
    let params = ParamBuilder::new().page(1).post_status(PostStatus::Any);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
    assert!(!posts
        .posts
        .iter()
        .all(|p| p.post_status == PostStatus::Publish));
}
