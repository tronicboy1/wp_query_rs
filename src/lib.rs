use query::params::Params;
use wp_post::WP_Post;

pub mod param_builder;
mod query;
mod sql;
mod wp_post;
mod query_builder;

#[allow(non_camel_case_types)]
pub struct WP_Query {
    pub posts: Vec<WP_Post>,
}

impl WP_Query {
    pub fn new(params: Params) -> Self {
        Self { posts: vec![] }
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
