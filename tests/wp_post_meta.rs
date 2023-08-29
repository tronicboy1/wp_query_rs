use std::time::{SystemTime, UNIX_EPOCH};
use wp_query_rs::{wp_post::WpMetaResults, *};

fn add_post() -> u64 {
    let mut post = WP_Post::new(1);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let title = format!("My Test Meta Post {}", now);
    post.post_title = title.clone();

    post.insert().expect("InsertFailed");

    let q = ParamBuilder::new().s(&title);
    let query = WP_Query::new(q).unwrap();

    query.posts.first().unwrap().ID
}

#[test]
fn can_add_post_meta() {
    let post_id = add_post();

    add_post_meta(post_id, "my_custom_rs_meta", 42).expect("MetaInsertFailed");
}

#[test]
fn can_get_post_meta() {
    let post_id = add_post();

    add_post_meta(post_id, "my_custom_rs_meta", 42).expect("MetaInsertFailed");

    let meta = get_post_meta(post_id, "my_custom_rs_meta", true);

    match meta {
        WpMetaResults::Single(meta) => {
            assert_eq!(meta.meta_value, "42")
        }
        _ => panic!("MetaQueryFailed"),
    }
}
