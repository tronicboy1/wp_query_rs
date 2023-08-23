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
mod tests {}
