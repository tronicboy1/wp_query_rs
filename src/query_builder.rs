use std::{collections::HashMap, fmt::Display};

use crate::query::{taxonomy::TaxonomyRelation, Query};

pub struct QueryBuilder<T>
where
    T: Display,
{
    pub query: Query<T>,
}

#[allow(non_snake_case)]
impl<T> QueryBuilder<T>
where
    T: Display,
{
    pub fn new() -> Self {
        QueryBuilder {
            query: Query::default(),
        }
    }

    pub fn author(mut self, author_id: usize) -> Self {
        self.query.author = Some(author_id);

        self
    }

    pub fn author_name(mut self, s: String) -> Self {
        self.query.author_name = Some(s);

        self
    }

    fn author__in(mut self) -> Self {
        self
    }
    fn author__not_in(mut self) -> Self {
        self
    }

    pub fn cat(mut self, cat_id: u32) -> Self {
        self.query.cat = Some(cat_id);

        self
    }

    pub fn category_name(mut self, s: String) -> Self {
        self.query.category_name = Some(s);

        self
    }

    fn category__and(mut self) -> Self {
        self
    }

    fn category__in(mut self) -> Self {
        self
    }

    fn category__not_in(mut self) -> Self {
        self
    }

    pub fn tag(mut self, slug: String) -> Self {
        self.query.tag = Some(slug);

        self
    }

    pub fn tag_id(mut self, tag_id: u32) -> Self {
        self.query.tag_id = Some(tag_id);

        self
    }

    fn tag__and(mut self) -> Self {
        self
    }

    fn tag__in(mut self) -> Self {
        self
    }

    fn tag__not_in(mut self) -> Self {
        self
    }

    fn tag_slug__and(mut self) -> Self {
        self
    }

    fn tag_slug__in(mut self) -> Self {
        self
    }

    pub fn tax_query(mut self, query: TaxonomyRelation<T>, relation: Option<String>) -> Self {
        let mut tax_q = self.query.tax_query.unwrap_or(HashMap::new());

        let size = tax_q.len();

        if let Some(rel) = relation {
            let qs_for_relation = tax_q.entry(rel).or_insert(vec![]);
            qs_for_relation.push(query);

            self.query.tax_query = Some(tax_q);
        } else {
            self.query.tax_query = Some(TaxonomyRelation::new_single_tax_map(query));
        }

        self
    }
    pub fn p(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn name(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn page_id(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn pagename(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post_parent(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post_parent__in(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post_parent__not_in(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post__in(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post__not_in(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post_name__in(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post_password(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post_type(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post_status(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn comment_count(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn posts_per_page(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn page(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn ignore_sticky_posts(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn order(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn orderby(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn year(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn monthnum(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn w(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn day(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn hour(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn minute(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn second(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn m(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn meta_key(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn meta_value(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn meta_value_num(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn meta_compare(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn meta_query(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
    pub fn post_mime_type(mut self) -> Self {
        self.query.author = Some(author_id);

        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_add_author() {}

    #[test]
    fn can_add_category() {}

    #[test]
    fn can_add_tags() {}

    #[test]
    fn can_add_single_tax() {}

    #[test]
    fn can_add_multiple_tax() {}

    #[test]
    fn can_add_post_params() {}

    #[test]
    fn can_add_post_type() {}

    #[test]
    fn can_add_comment_params() {}

    #[test]
    fn can_add_pagination_params() {}

    #[test]
    fn can_add_orderby_params() {}

    #[test]
    fn can_add_date_params() {}
}
