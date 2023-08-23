# wp_query_rs

A rust implementation of the classic WP_Query utility to access WordPress posts outside of a WordPress environment.

# Example

If you have the environment variables setup (described below), you can use the new function to query similar to the WP_Query instance creation.

```rust
use wp_query_rs::{WP_Query, ParamBuilder};

let params = ParamBuilder::new();

let wp_query = WP_Query::new(params.params()).expect("SqlFailed");
assert_eq!(wp_query.post_count(), 10);
```

The prefered method of use would be to provide a connection from your own connection pool:

```rust
let params = ParamBuilder::new();

let mut con: mysql::Conn = pool.get_connection();

let wp_query = WP_Query::with_connection(&mut conn, params.params()).expect("SqlFailed");

assert_eq!(wp_query.post_count(), 10);
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

# Goals

The author of this package would like to add tooling to the rust community for working with WordPress websites and data. In the future, possibly even building extensions for WordPress written in Rust to increase performance.

The building of this library itself was largely for the author's educational purposes. If there is interest in software like this I would love to work with others to improve.

# Reference

The public APIs in this crate mostly model the functionality of the original WP_Query. You can see it's documentation for more information.

https://developer.wordpress.org/reference/classes/wp_query/#date-parameters

# License

- Apache License, Version 2.0 (LICENSE_APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE_MIT or http://opensource.org/licenses/MIT)
