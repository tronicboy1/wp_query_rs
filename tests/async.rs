use wp_query_rs::*;

#[cfg(feature = "query_async")]
#[tokio::test]
async fn can_query() {
    let params = ParamBuilder::new();

    let posts = WP_Query::new(params).await.expect("SqlFailed");

    assert_eq!(posts.post_count(), 10);
}
