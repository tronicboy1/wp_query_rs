use std::fmt::Display;

use mysql::{OptsBuilder, PooledConn};

use self::{env_vars::EnvVars, pool::get_pool};

pub mod env_vars;
pub mod pool;

fn build_opts_from_env(env_vars: EnvVars) -> OptsBuilder {
    OptsBuilder::new()
        .user(env_vars.user)
        .ip_or_hostname(env_vars.host)
        .pass(env_vars.password)
        .db_name(env_vars.db_name)
        .tcp_port(env_vars.port.unwrap_or(3306))
        .prefer_socket(true)
}

pub fn get_conn() -> Result<PooledConn, mysql::Error> {
    get_pool().get_conn()
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
