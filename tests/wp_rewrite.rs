use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn can_retrieve_options() {
    let rewrite = wp_rewrite::WpRewrite::new();

    let rewrite_rules = rewrite.wp_rewrite_rules().unwrap();
    assert!(rewrite_rules.is_some());
}

#[cfg(all(feature = "query_sync", feature = "rewrite"))]
#[test]
fn can_parse_url() {
    let rewrite = wp_rewrite::WpRewrite::new();

    let parsed = parse_request(
        &rewrite,
        url::Url::parse("http://localhost:8080/a-page-about-tomates/").unwrap(),
    )
    .unwrap();

    let params = Params::try_from(&parsed).unwrap();

    assert_eq!(params.name, Some("a-page-about-tomates"));
}
