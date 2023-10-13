use wp_query_rs::*;

#[cfg(feature = "query_async")]
#[test]
fn can_query() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let params = ParamBuilder::new();

        let posts = WP_Query::new(params).await.expect("SqlFailed");

        assert_eq!(posts.post_count(), 10);
    });
}
