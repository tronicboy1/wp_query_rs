use mysql::prelude::Queryable;
use mysql_common::time::PrimitiveDateTime;
use serde::ser::SerializeStruct;

use crate::sql::{date::get_date_now, find_col, get_conn};

#[derive(Debug)]
pub struct WpUser {
    pub id: u64,
    user_login: String,
    _user_pass: String,
    pub user_nicename: String,
    _user_email: String,
    pub user_url: String,
    pub user_registered: PrimitiveDateTime,
    _user_activation_key: String,
    _user_status: i64,
    pub display_name: String,
}

impl WpUser {
    pub fn get_user_by_id(id: u64) -> Result<Option<Self>, mysql::Error> {
        let mut conn = get_conn()?;

        let stmt = conn.prep("SELECT * FROM wp_users WHERE ID = ?")?;

        let value = mysql::Value::UInt(id);

        conn.exec_first(stmt, mysql::Params::Positional(vec![value]))
            .map(|row: Option<mysql::Row>| row.map(|r| WpUser::from(r)))
    }
}

impl From<mysql::Row> for WpUser {
    fn from(mut value: mysql::Row) -> Self {
        WpUser {
            id: find_col(&mut value, "ID").unwrap_or(0),
            user_login: find_col(&mut value, "user_login").unwrap_or(String::new()),
            _user_pass: find_col(&mut value, "user_pass").unwrap_or(String::new()),
            user_nicename: find_col(&mut value, "user_nicename").unwrap_or(String::new()),
            _user_email: find_col(&mut value, "user_email").unwrap_or(String::new()),
            user_url: find_col(&mut value, "user_url").unwrap_or(String::new()),
            user_registered: find_col(&mut value, "user_registered").unwrap_or(get_date_now()),
            _user_activation_key: find_col(&mut value, "user_activation_key")
                .unwrap_or(String::new()),
            _user_status: find_col(&mut value, "user_status").unwrap_or(0i64),
            display_name: find_col(&mut value, "display_name").unwrap_or(String::new()),
        }
    }
}

impl serde::Serialize for WpUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("WpUser", 6)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("user_login", &self.user_login)?;
        state.serialize_field("user_nicename", &self.user_nicename)?;
        state.serialize_field("user_url", &self.user_url)?;
        state.serialize_field("user_registered", &self.user_registered.to_string())?;
        state.serialize_field("display_name", &self.display_name)?;

        state.end()
    }
}
