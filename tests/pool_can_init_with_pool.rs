use wp_query_rs::*;

// Must be in separate file so that the default init is not called
#[cfg(feature = "query_async")]
#[tokio::test]
async fn init_with_my_pool() {
    use std::env;

    let host = env::var("WORDPRESS_DB_HOST").unwrap();
    let user = env::var("WORDPRESS_DB_USER").unwrap();
    let password = env::var("WORDPRESS_DB_PASSWORD").unwrap();
    let name = env::var("WORDPRESS_DB_NAME").unwrap();

    let pool = mysql_async::Pool::new(
        mysql_async::OptsBuilder::default()
            .ip_or_hostname(host)
            .user(Some(user))
            .pass(Some(password))
            .db_name(Some(name)),
    );
    PoolInit::with_pool(&pool).expect("Pool was set before call");

    let params = ParamBuilder::new();

    let posts = WP_Query::new(params).await.expect("SqlFailed");

    assert_eq!(posts.post_count(), 10);
}

#[cfg(feature = "query_sync")]
#[test]
fn init_with_my_pool() {
    use std::env;

    let host = env::var("WORDPRESS_DB_HOST").unwrap();
    let user = env::var("WORDPRESS_DB_USER").unwrap();
    let password = env::var("WORDPRESS_DB_PASSWORD").unwrap();
    let name = env::var("WORDPRESS_DB_NAME").unwrap();

    let pool = mysql::Pool::new(
        mysql::OptsBuilder::default()
            .ip_or_hostname(Some(host))
            .user(Some(user))
            .pass(Some(password))
            .db_name(Some(name)),
    ).unwrap();
    PoolInit::with_pool(&pool).expect("Pool was set before call");

    let params = ParamBuilder::new();

    let posts = WP_Query::new(params).expect("SqlFailed");

    assert_eq!(posts.post_count(), 10);
}
