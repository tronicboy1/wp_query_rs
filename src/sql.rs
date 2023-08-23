use std::{fmt::Display, str::FromStr};

use mysql::{OptsBuilder, Pool};
use mysql_common::{time::Date, FromValueError, Row};

use crate::{wp_post::post_status::PostStatus, WP_Post};

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

pub fn unwrap_row(row: &mut Row) -> Result<WP_Post, FromValueError> {
    let id: u64 = row.take_opt(0).unwrap()?;
    let post_author: u64 = row.take_opt(1).unwrap()?;
    let comment_count: u64 = row.take_opt(2).unwrap()?;
    let post_parent: u64 = row.take_opt(3).unwrap()?;
    let menu_order: u64 = row.take_opt(4).unwrap()?;
    let post_date: Date = row.take_opt(5).unwrap()?;
    let post_date_gmt: Date = row.take_opt(6).unwrap().unwrap_or(post_date.clone());
    let post_modified: Date = row.take_opt(7).unwrap().unwrap_or(post_date.clone());
    let post_modified_gmt: Date = row.take_opt(8).unwrap().unwrap_or(post_date_gmt.clone());
    let post_status: String = row.take_opt(9).unwrap()?;
    let post_status = PostStatus::from_str(&post_status).unwrap();
    let post_content: String = row.take_opt(10).unwrap()?;
    let post_title: String = row.take_opt(11).unwrap()?;
    let post_excerpt: String = row.take_opt(12).unwrap()?;
    let comment_status: String = row.take_opt(13).unwrap()?;
    let ping_status: String = row.take_opt(14).unwrap()?;
    let post_password: String = row.take_opt(15).unwrap()?;
    let post_name: String = row.take_opt(16).unwrap()?;
    let to_ping: String = row.take_opt(17).unwrap()?;
    let pinged: String = row.take_opt(18).unwrap()?;
    let post_content_filtered: String = row.take_opt(19).unwrap()?;
    let guid: String = row.take_opt(20).unwrap()?;
    let post_type: String = row.take_opt(21).unwrap()?;
    let post_mime_type: String = row.take_opt(22).unwrap()?;

    Ok(WP_Post {
        ID: id,
        post_author,
        post_date,
        post_date_gmt,
        post_content,
        post_title,
        post_excerpt,
        post_status,
        comment_status,
        ping_status,
        post_password,
        post_name,
        to_ping,
        pinged,
        post_modified,
        post_modified_gmt,
        post_content_filtered,
        post_parent,
        guid,
        menu_order,
        post_type,
        post_mime_type,
        comment_count,
    })
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
    In,
    NotIn,
    Between,
    NotBetween,
    NotExists,
    Regexp,
    NotRegexp,
    Rlike,
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
