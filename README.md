# wp_query_rs

A rust implementation of the classic WP_Query utility to access WordPress posts outside of a WordPress environment.

# Installation

For a typical installation, you can add the following to Cargo.toml to get started:

```toml
wp_query_rs = {version = "*", features = ["query_sync"] }
```

This will enable global connection to a mysql database (initiates a connection pool in a OnceBlock, in future release will make it possible to use without forcing this).

## Async

This library partially supports async mysql operations. We can only guarantee compatibility with the Tokio runtime.

```toml
wp_query_rs = {version = "*", features = ["query_async"] }
```

# Example

If you have the environment variables setup (described below), you can use the new function to query similar to the WP_Query instance creation.

```rust
use wp_query_rs::{WP_Query, ParamBuilder};

let params = ParamBuilder::new();

let wp_query = WP_Query::new(params).expect("SqlFailed");
assert_eq!(wp_query.post_count(), 10);
```

The prefered method of use would be to provide a connection from your own connection pool:

```rust
let params = ParamBuilder::new();

let mut con: mysql::Conn = pool.get_connection();

let wp_query = WP_Query::with_connection(&mut conn, params).expect("SqlFailed");

assert_eq!(wp_query.post_count(), 10);
```

# Initialization

By default, this library will produce it's own global connection pool to allow the API to mimick that of the WordPress version.
The default initialization will use the following environment variables to connect:

- WORDPRESS_DB_HOST
- WORDPRESS_DB_USER
- WORDPRESS_DB_PASSWORD
- WORDPRESS_DB_NAME
- WORDPRESS_DB_PORT

## Implicitly Initializing Global Pool

The default functionality of initializing a global connection pool can be very inconvenient if you have a pool already created,
or would like to provide your own custom options to initialize.

In such cases we can use the `PoolInit::with_opts` or `PoolInit::with_pool` to set the global pool reference implicitly.

Ensure that you call either function before calling any of this librarys database calling methods.

### with_opts

```rust
let opts = mysql_async::OptsBuilder::default();

PoolInit::with_opts(opts.into()).expect("Pool was set before call");
```

### with_pool

```rust
let pool = mysql_async::Pool::new(
        mysql_async::OptsBuilder::default(),
);

PoolInit::with_pool(&pool).expect("Pool was set before call");
```

# Building Query Parameters

To add parameters to your query, chain the callbacks after `ParamBuilder::new()`:

```rust
let params = ParamBuilder::new().posts_per_page(2).page(3);
```

The order of chaining is irrelevant.

# Panics!

The param builder will panic if you supply illegal date parameters:

```rust
let params = ParamBuilder::new()
        .year(2023)
        .monthnum(1)
        .day(1)
        .hour(4)
        .minute(23)
        .second(61); // Panics!
```

# Inserting Posts

This package can also be used to insert posts into WordPress. This functionality uses the default connection pool initiated by this package.

```rust
let mut post = WP_Post::new(1);
let title = "My Test Post".to_string();
post.post_title = title.clone();

let post_id: u64 = post.insert().expect("InsertFailed");
```

# Reading and Writing Meta Data

You can also read and write metadata.

```rust
let post_id: u64 = post.insert().expect("InsertFailed");

add_post_meta(post_id, "my_custom_rs_meta", 42).expect("MetaInsertFailed");

let meta = get_post_meta(post_id, "my_custom_rs_meta", true);

match meta {
WpMetaResults::Single(meta) => {
        assert_eq!(meta.meta_value, "42")
}
_ => unreachable!("MetaQueryFailed"),
}
```

# Reading WP User Data

You can load WP User data from the database usin `WpUser` as well.

```rust
let user = WpUser::get_user_by_id(1).unwrap().unwrap();

assert_eq!(user.id, 1);
```

# Parsing WordPress Pretty URLs into WP_Query Parameters

**This feature is highly EXPERIMENTAL!**

By enabling the `rewrite` feature of this crate, you can parse WordPress pretty URIs/slugs into WpQuery Parameters for `wp_query_rs`.

```rust
use wp_query_rs::wp_rewrite::{WpRewrite, parse_request};
use url::Url;

let rewrite = wp_rewrite::WpRewrite::new();

let parsed = parse_request(
        &rewrite,
        Url::parse("http://localhost:8080/a-page-about-tomates/").unwrap(),
)
.unwrap();

let params = Params::try_from(&parsed).unwrap();

assert_eq!(params.name, Some("a-page-about-tomates"));
```

# Goals

The author of this package would like to add tooling to the rust community for working with WordPress websites and data. In the future, possibly even building extensions for WordPress written in Rust to increase performance.

The building of this library itself was largely for the author's educational purposes. If there is interest in software like this I would love to work with others to improve.

# Reference

The public APIs in this crate mostly model the functionality of the original WP_Query. You can see it's documentation for more information.

https://developer.wordpress.org/reference/classes/wp_query/#date-parameters

# License

- Apache License, Version 2.0 (LICENSE_APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE_MIT or http://opensource.org/licenses/MIT)
