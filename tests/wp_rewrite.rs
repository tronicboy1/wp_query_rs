use wp_query_rs::*;

#[cfg(feature = "rewrite")]
#[test]
fn can_retrieve_options() {
    let rewrite = wp_rewrite::WpRewrite::new();

    let rewrite_rules = rewrite.wp_rewrite_rules().unwrap();
    assert!(rewrite_rules.is_some());
}
