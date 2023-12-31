use std::collections::HashMap;

use wp_query_rs::*;

pub fn ensure_no_duplicate(posts: &[WP_Post]) {
    let all_counts_less_than_one = posts
        .iter()
        .fold(HashMap::new(), |mut acc, post| {
            let c = acc.entry(post.ID).or_insert(0);
            *c += 1;

            acc
        })
        .values()
        .all(|c| *c == 1);
    assert!(all_counts_less_than_one, "Duplicates found!")
}

#[cfg(feature = "query_sync")]
#[test]
fn default_selects_posts() {
    let params = ParamBuilder::new();

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn no_duplicate_ids() {
    let params = ParamBuilder::new();
    let posts = WP_Query::new(params).expect("SqlFailed");
    ensure_no_duplicate(&posts.posts);
}

#[cfg(feature = "query_sync")]
#[test]
fn no_dups_on_term_join() {
    let params = ParamBuilder::new().tag_id(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    ensure_no_duplicate(&posts.posts);
}

#[cfg(feature = "query_sync")]
#[test]
fn meta_data() {
    let params = ParamBuilder::new().meta_value("1");

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);
}

#[cfg(feature = "query_sync")]
#[test]
fn meta_join_no_dups() {
    let params = ParamBuilder::new().meta_value("1");

    let posts = WP_Query::new(params).expect("SqlFailed");
    ensure_no_duplicate(&posts.posts);
}

#[cfg(feature = "query_sync")]
#[test]
fn paginate() {
    let params = ParamBuilder::new().page(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() > 0);

    let params = ParamBuilder::new().posts_per_page(2);

    let posts = WP_Query::new(params).expect("SqlFailed");
    assert!(posts.post_count() == 2);
}
