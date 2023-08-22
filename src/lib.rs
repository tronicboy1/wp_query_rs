use mysql::Error;
use query::params::Params;
use sql::get_pool;
use wp_post::WP_Post;

pub mod param_builder;
mod query;
mod query_builder;
mod sql;
mod wp_post;

pub use param_builder::ParamBuilder;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct WP_Query {
    pub posts: Vec<WP_Post>,
}

impl WP_Query {
    pub fn new(params: Params) -> Result<Self, Error> {
        let pool = get_pool(sql::env_vars::EnvVars::from_env())?;

        Ok(Self { posts: vec![] })
    }

    pub fn post_count(&self) -> usize {
        self.posts.len()
    }

    pub fn max_num_pages(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
