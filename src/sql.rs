use mysql::{OptsBuilder, Pool};

use self::env_vars::EnvVars;

pub mod env_vars;

pub fn get_pool(env_vars: EnvVars) -> Result<Pool, mysql::Error> {
    let opts = OptsBuilder::new()
        .user(env_vars.user)
        .ip_or_hostname(env_vars.host)
        .pass(env_vars.password)
        .db_name(env_vars.db_name)
        .tcp_port(env_vars.port.unwrap_or(3306))
        .prefer_socket(true);

    Pool::new(opts)
}

#[derive(Debug, PartialEq, Eq)]
pub enum SqlConditionOperator {
    In,
    NotIn,
    And,
    Exists,
    NotExists,
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
    In,
    NotIn,
    Between,
    NotBetween,
    NotExists,
    Regexp,
    NotRegexp,
    Rlike,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SqlOrder {
    Asc,
    Desc,
}
