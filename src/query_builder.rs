use std::fmt::Display;

use crate::query::Query;

pub struct QueryBuilder<T>
where
    T: Display,
{
    pub query: Query<T>,
}

impl<T> QueryBuilder<T>
where
    T: Display,
{
    fn new() -> Self {
        QueryBuilder {
            query: Query::default(),
        }
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
