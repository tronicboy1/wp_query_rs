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

#[cfg(test)]
mod tests {}
