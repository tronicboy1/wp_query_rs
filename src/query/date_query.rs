use mysql_common::prelude::ToValue;

use crate::sql::SqlConditionOperator;

pub struct DateQuery {
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub week: Option<u8>,   // Week of the year (from 0 to 53).
    pub day: Option<u8>,    //Day of the month (from 1 to 31).
    pub hour: Option<u8>,   // Hour (from 0 to 23).
    pub minute: Option<u8>, // Minute (from 0 to 59).
    pub second: Option<u8>,
    pub after: Option<DateQueryAfterBefore>,
    pub before: Option<DateQueryAfterBefore>,
    pub inclusive: bool,
    pub column: String,
    pub relation: SqlConditionOperator,
}

impl DateQuery {
    pub fn new() -> Self {
        Self {
            year: None,
            month: None,
            week: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            after: None,
            before: None,
            inclusive: false,
            column: String::from("post_date"),
            relation: SqlConditionOperator::And,
        }
    }
}

pub struct DateQueryAfterBefore {
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>, // Week of the year (from 0 to 53).
}

impl ToValue for DateQuery {
    fn to_value(&self) -> mysql_common::Value {
        mysql_common::Value::Date(
            self.year.unwrap_or(2023),
            self.month.unwrap_or(1),
            self.day.unwrap_or(1),
            self.hour.unwrap_or(0),
            self.minute.unwrap_or(0),
            self.second.unwrap_or(0),
            0u32,
        )
    }
}

impl ToValue for DateQueryAfterBefore {
    fn to_value(&self) -> mysql_common::Value {
        mysql_common::Value::Date(
            self.year.unwrap_or(2023),
            self.month.unwrap_or(1),
            self.day.unwrap_or(1),
            0u8,
            0u8,
            0u8,
            0u32,
        )
    }
}
