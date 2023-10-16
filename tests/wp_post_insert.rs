use std::time::{SystemTime, UNIX_EPOCH};
use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn can_insert_post() {
    let mut post = WP_Post::new(1);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let title = format!("My Test Post {}", now);
    post.post_title = title.clone();

    post.insert().expect("InsertFailed");

    let q = ParamBuilder::new().s(&title);
    let query = WP_Query::new(q).unwrap();

    assert!(query.post_count() > 0);
}

#[cfg(feature = "query_async")]
#[tokio::test]
async fn can_insert_post() {
    let mut post = WP_Post::new(1);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let title = format!("My Test Post {}", now);
    post.post_title = title.clone();

    post.insert().await.expect("InsertFailed");

    let q = ParamBuilder::new().s(&title);
    let query = WP_Query::new(q).await.unwrap();

    assert!(query.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn post_id_returned() {
    let mut post = WP_Post::new(1);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let title = format!("My Test Post {}", now);
    post.post_title = title.clone();

    let post_id = post.insert().expect("InsertFailed");
    assert!(post_id > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn can_insert_multiple_posts() {
    let dirty_posts: Vec<WP_Post> = (0..10)
        .map(|_| {
            let mut post = WP_Post::new(1);
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            post.post_title = format!("My Bulk Posts {}", now);

            post
        })
        .collect();

    WP_Post::insert_bulk(dirty_posts).expect("BulkInsertFailed");

    let q = ParamBuilder::new().s("My Bulk Posts");
    let query = WP_Query::new(q).unwrap();

    assert!(query.post_count() == 10);
}

#[cfg(feature = "query_async")]
#[tokio::test]
async fn can_insert_multiple_posts() {
    let dirty_posts: Vec<WP_Post> = (0..10)
        .map(|_| {
            let mut post = WP_Post::new(1);
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            post.post_title = format!("My Bulk Posts {}", now);

            post
        })
        .collect();

    WP_Post::batch(dirty_posts).await.expect("BulkInsertFailed");

    let q = ParamBuilder::new().s("My Bulk Posts");
    let query = WP_Query::new(q).await.unwrap();

    assert!(query.post_count() == 10);
}
