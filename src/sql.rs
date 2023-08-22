mod env_vars;

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
