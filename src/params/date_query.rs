use std::fmt::Display;

use mysql_common::prelude::ToValue;

use crate::sql::SqlConditionOperator;

#[derive(Debug)]
pub struct DateQuery {
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>,    //Day of the month (from 1 to 31).
    pub hour: Option<u8>,   // Hour (from 0 to 23).
    pub minute: Option<u8>, // Minute (from 0 to 59).
    pub second: Option<u8>,
    pub after: Option<DateQueryAfterBefore>,
    pub before: Option<DateQueryAfterBefore>,
    pub inclusive: bool,
    pub column: DateColumn,
    pub relation: SqlConditionOperator,
}

impl DateQuery {
    pub fn new() -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
            after: None,
            before: None,
            inclusive: false,
            column: DateColumn::PostDate,
            relation: SqlConditionOperator::And,
        }
    }

    pub fn year(mut self, y: u16) -> Self {
        self.year = Some(y);

        self
    }

    pub fn month(mut self, m: u8) -> Self {
        self.month = Some(m);

        self
    }

    pub fn day(mut self, d: u8) -> Self {
        self.day = Some(d);

        self
    }

    pub fn hour(mut self, h: u8) -> Self {
        self.hour = Some(h);

        self
    }

    pub fn minute(mut self, m: u8) -> Self {
        self.minute = Some(m);

        self
    }

    pub fn second(mut self, s: u8) -> Self {
        self.second = Some(s);

        self
    }

    pub fn after(mut self, after: DateQueryAfterBefore) -> Self {
        self.after = Some(after);

        self
    }

    pub fn before(mut self, before: DateQueryAfterBefore) -> Self {
        self.before = Some(before);

        self
    }

    pub fn inclusive(mut self, inclusive: bool) -> Self {
        self.inclusive = inclusive;

        self
    }

    pub fn column(mut self, col: DateColumn) -> Self {
        self.column = col;

        self
    }

    pub fn relation(mut self, rel: SqlConditionOperator) -> Self {
        let rel = match rel {
            SqlConditionOperator::And => rel,
            SqlConditionOperator::Or => rel,
            _ => SqlConditionOperator::And,
        };

        self.relation = rel;

        self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DateColumn {
    PostDate,
    PostDateGmt,
    ModifiedDate,
    ModifiedDateGmt,
}

impl Display for DateColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PostDate => "post_date",
                Self::PostDateGmt => "post_date_gmt",
                Self::ModifiedDate => "post_modified",
                Self::ModifiedDateGmt => "post_modified_gmt",
            }
        )
    }
}

#[derive(Debug)]
pub struct DateQueryAfterBefore {
    pub year: u16,
    pub month: u8,
    pub day: u8, // Week of the year (from 0 to 53).
}

impl DateQueryAfterBefore {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
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
        mysql_common::Value::Date(self.year, self.month, self.day, 0u8, 0u8, 0u8, 0u32)
    }
}

impl ToValue for DateColumn {
    fn to_value(&self) -> mysql_common::Value {
        let str = match self {
            Self::PostDate => "post_date",
            Self::PostDateGmt => "post_date_gmt",
            Self::ModifiedDate => "post_modified",
            Self::ModifiedDateGmt => "post_modified_gmt",
        };

        mysql_common::Value::Bytes(str.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_year() {
        let dq = DateQuery::new().year(2022);
        assert_eq!(dq.year.unwrap(), 2022);
    }

    #[test]
    fn can_add_month() {
        let dq = DateQuery::new().month(2);
        assert_eq!(dq.month.unwrap(), 2);
    }

    #[test]
    fn can_add_day() {
        let dq = DateQuery::new().day(2);
        assert_eq!(dq.day.unwrap(), 2);
    }

    #[test]
    fn can_add_minute() {
        let dq = DateQuery::new().minute(2);
        assert_eq!(dq.minute.unwrap(), 2);
    }

    #[test]
    fn can_add_second() {
        let dq = DateQuery::new().second(2);
        assert_eq!(dq.second.unwrap(), 2);
    }

    #[test]
    fn can_add_after() {
        let dq = DateQuery::new().after(DateQueryAfterBefore::new(2022, 2, 3));
        let after = dq.after.unwrap();
        assert_eq!(after.day, 3);
        assert_eq!(after.month, 2);
        assert_eq!(after.year, 2022);
    }

    #[test]
    fn can_add_before() {
        let dq = DateQuery::new().before(DateQueryAfterBefore::new(2022, 2, 3));
        let before = dq.before.unwrap();
        assert_eq!(before.day, 3);
        assert_eq!(before.month, 2);
        assert_eq!(before.year, 2022);
    }

    #[test]
    fn can_set_inclusive() {
        let dq = DateQuery::new().inclusive(true);
        assert!(dq.inclusive);
    }

    #[test]
    fn can_set_column() {
        let dq = DateQuery::new().column(DateColumn::ModifiedDateGmt);
        assert_eq!(dq.column, DateColumn::ModifiedDateGmt);
    }
}
