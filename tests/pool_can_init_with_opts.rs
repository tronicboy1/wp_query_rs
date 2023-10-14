use wp_query_rs::*;

// Must be in separate file so that the default init is not called
#[cfg(feature = "query_async")]
#[tokio::test]
async fn init_with_my_opts() {
    use std::env;

    let host = env::var("WORDPRESS_DB_HOST").unwrap();
    let user = env::var("WORDPRESS_DB_USER").unwrap();
    let password = env::var("WORDPRESS_DB_PASSWORD").unwrap();
    let name = env::var("WORDPRESS_DB_NAME").unwrap();

    let opts = mysql_async::OptsBuilder::default()
        .ip_or_hostname(host)
        .user(Some(user))
        .pass(Some(password))
        .db_name(Some(name));
    PoolInit::with_opts(opts.into()).expect("Pool was set before call");

    let params = ParamBuilder::new();

    let posts = WP_Query::new(params).await.expect("SqlFailed");

    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn init_with_my_opts() {
    use std::env;

    let host = env::var("WORDPRESS_DB_HOST").unwrap();
    let user = env::var("WORDPRESS_DB_USER").unwrap();
    let password = env::var("WORDPRESS_DB_PASSWORD").unwrap();
    let name = env::var("WORDPRESS_DB_NAME").unwrap();

    let opts = mysql::OptsBuilder::default()
        .ip_or_hostname(Some(host))
        .user(Some(user))
        .pass(Some(password))
        .db_name(Some(name));
    PoolInit::with_opts(opts.into()).expect("Pool was set before call");

    let params = ParamBuilder::new();

    let posts = WP_Query::new(params).expect("SqlFailed");

    assert_eq!(posts.post_count(), 10);
}
