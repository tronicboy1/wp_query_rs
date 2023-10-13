use std::fmt::Display;

use mysql_common::prelude::FromValue;

pub mod cast_type;
pub mod date;
pub mod env_vars;
pub mod pool;
pub mod traits;
use self::env_vars::EnvVars;
use self::pool::get_pool;
#[cfg(feature = "query_sync")]
use mysql::{OptsBuilder, PooledConn};

#[cfg(feature = "query_async")]
use mysql_async::OptsBuilder;
#[cfg(feature = "query_sync")]
fn build_opts_from_env(env_vars: EnvVars) -> OptsBuilder {
    OptsBuilder::new()
        .user(env_vars.user)
        .ip_or_hostname(env_vars.host)
        .pass(env_vars.password)
        .db_name(env_vars.db_name)
        .tcp_port(env_vars.port.unwrap_or(3306))
        .prefer_socket(true)
}
#[cfg(feature = "query_async")]
fn build_opts_from_env(env_vars: EnvVars) -> OptsBuilder {
    OptsBuilder::default()
        .user(env_vars.user)
        .ip_or_hostname(env_vars.host.expect("must define mysql host"))
        .pass(env_vars.password)
        .db_name(env_vars.db_name)
        .tcp_port(env_vars.port.unwrap_or(3306))
        .prefer_socket(true)
}

#[cfg(feature = "query_sync")]
pub fn get_conn() -> Result<PooledConn, mysql::Error> {
    get_pool().get_conn()
}
#[cfg(feature = "query_async")]
pub async fn get_conn() -> Result<mysql_async::Conn, mysql_async::Error> {
    get_pool().get_conn().await
}

#[derive(Debug, PartialEq, Eq)]
pub enum SqlConditionOperator {
    In,
    NotIn,
    And,
    Exists,
    NotExists,
    Or,
}

impl Display for SqlConditionOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "AND",
                Self::Exists => "EXISTS",
                Self::In => "IN",
                Self::NotExists => "NOT EXISTS",
                Self::NotIn => "NOT IN",
                Self::Or => "OR",
            }
        )
    }
}

#[derive(Debug)]
pub enum SqlCompareOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SqlSearchOperators {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Like,
    NotLike,
    Exists,
    NotExists,
}

impl Display for SqlSearchOperators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equals => "=",
                Self::NotEquals => "!=",
                Self::GreaterThan => ">",
                Self::LessThan => "<",
                Self::GreaterThanOrEqualTo => ">=",
                Self::LessThanOrEqualTo => "<=",
                Self::Like => "LIKE",
                Self::NotLike => "NOT LIKE",
                Self::Exists => "EXISTS",
                Self::NotExists => "NOT EXISTS",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlOrder {
    Asc,
    Desc,
}

impl ToString for SqlOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Asc => String::from("ASC"),
            Self::Desc => String::from("DESC"),
        }
    }
}

impl Into<SqlOrder> for &str {
    fn into(self) -> SqlOrder {
        match self.to_uppercase().as_str() {
            "ASC" => SqlOrder::Asc,
            "DESC" => SqlOrder::Desc,
            _ => SqlOrder::Desc,
        }
    }
}

/// Finds a value T in a row by it's column name
///
/// # Example
/// ```rust,ignore
/// let id: u64 = find_col(&mut value, "ID").unwrap_or(0);
/// ```
pub fn find_col<T>(row: &mut mysql_common::Row, col_name: &str) -> Option<T>
where
    T: FromValue,
{
    let (i, ..) = row
        .columns_ref()
        .iter()
        .enumerate()
        .find(|(_, col)| col.name_str() == col_name)?;

    row.take_opt(i).map(|r| r.ok()).flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_order_from_str() {
        let str = "ASC";
        let order: SqlOrder = str.into();

        assert_eq!(order, SqlOrder::Asc);

        let str = "DESC";
        let order: SqlOrder = str.into();

        assert_eq!(order, SqlOrder::Desc);
    }

    #[test]
    fn can_convert_order_from_str_undercase_mixed() {
        let str = "Asc";
        let order: SqlOrder = str.into();

        assert_eq!(order, SqlOrder::Asc);

        let str = "desC";
        let order: SqlOrder = str.into();

        assert_eq!(order, SqlOrder::Desc);
    }
}
