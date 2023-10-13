use wp_query_rs::wp_rewrite::parse_request;
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

#[cfg(feature = "query_async")]
#[test]
fn can_retrieve_options() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let rewrite = wp_rewrite::WpRewrite::new();

        let rewrite_rules = rewrite.wp_rewrite_rules().await.unwrap();
        assert!(rewrite_rules.is_some());
    });
}

#[cfg(feature = "query_async")]
#[test]
fn can_parse_url() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let rewrite = wp_rewrite::WpRewrite::new();

        let parsed = parse_request(
            &rewrite,
            url::Url::parse("http://localhost:8080/a-page-about-tomates/").unwrap(),
        )
        .await
        .unwrap();

        let params = Params::try_from(&parsed).unwrap();

        assert_eq!(params.name, Some("a-page-about-tomates"));
    });
}
