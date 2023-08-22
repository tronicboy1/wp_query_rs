use wp_query_rs::*;

#[test]
fn default_selects_posts() {
  let params = ParamBuilder::new();

  let posts = WP_Query::new(params.params()).expect("SqlFailed");
  assert_eq!(posts.post_count(), 0);
}
