use std::error::Error;

use crate::query::params::Params;

pub struct QueryBuilder {
    query: String,
    value_count: u32,
}

impl QueryBuilder {
    fn new(params: Params) -> Self {
        Self {
            query: String::new(),
            value_count: 0,
        }
    }

    fn build_query(&mut self, params: Params) -> Result<(), Box<dyn Error>> {
        self.query.push_str(
            "SELECT ID,post_author,post_date,post_date_gmt,
        post_content,post_title,post_excerpt,post_status,comment_status,ping_status,
        post_password,post_name,to_ping,pinged,post_modified,post_modified_gmt,
        post_content_filtered,post_parent,guid,menu_order,post_type,post_mime_type,comment_count
        FROM wp_posts",
        );

        let join_term = check_if_term_join_necessary(&params);
        let join_meta = check_if_meta_join_necessary(&params);

        if join_meta {
            self.query
                .push_str(" INNER JOIN wp_postmeta ON wp_postmeta.post_id = wp_posts.ID");
        }

        if join_term {
            self.query.push_str(
                " INNER JOIN wp_term_relationships
            ON wp_posts.ID = wp_term_relationships.object_id
            INNER JOIN wp_terms ON wp_terms.term_id = wp_term_relationships.term_taxonomy_id",
            );
        }

        self.query.push_str(" LIMIT 5;");

        Ok(())
    }
}

fn check_if_term_join_necessary(params: &Params) -> bool {
    params.tag.is_some()
        || params.tag__and.is_some()
        || params.tag__in.is_some()
        || params.tag__not_in.is_some()
        || params.tag_id.is_some()
        || params.tag_slug__and.is_some()
        || params.tag_slug__in.is_some()
        || params.tax_query.is_some()
}

fn check_if_meta_join_necessary(params: &Params) -> bool {
    params.meta_key.is_some()
        || params.meta_value.is_some()
        || params.meta_value_num.is_some()
        || params.meta_query.is_some()
}
