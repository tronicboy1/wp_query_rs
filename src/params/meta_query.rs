use std::fmt::Display;

use crate::sql::SqlSearchOperators;

#[derive(Debug)]
pub struct MetaQuery {
    pub key: String,
    pub value: String,
    pub compare: SqlSearchOperators,
    // TODO
    // type
}

impl MetaQuery {
    pub fn new<T>(key: &str, value: T, compare: SqlSearchOperators) -> Self
    where
        T: Display,
    {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            compare,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MetaRelation {
    Or,
    And,
}

impl Display for MetaRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Or => "OR",
                Self::And => "AND",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_query_with_int() {
        let q = MetaQuery::new("my_meta", 42, SqlSearchOperators::NotEquals);

        assert_eq!(q.key, String::from("my_meta"));
        assert_eq!(q.value, String::from("42"));
        assert_eq!(q.compare, SqlSearchOperators::NotEquals);
    }

    #[test]
    fn can_create_new_query_with_str() {
        let q = MetaQuery::new("my_meta", "42", SqlSearchOperators::NotEquals);

        assert_eq!(q.key, String::from("my_meta"));
        assert_eq!(q.value, String::from("42"));
        assert_eq!(q.compare, SqlSearchOperators::NotEquals);
    }

    #[test]
    fn can_write_meta_relation_to_string() {
        assert_eq!(MetaRelation::And.to_string(), String::from("AND"));
        assert_eq!(MetaRelation::Or.to_string(), String::from("OR"));
    }
}
