use crate::sql::SqlSearchOperators;

#[derive(Debug)]
pub struct MetaQuery {
    pub key: String,
    pub value: String,
    pub compare: SqlSearchOperators,
    // TODO
    // type
}

#[cfg(test)]
mod tests {}
