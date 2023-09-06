use chrono::{Datelike, Timelike};
use mysql_common::time::{Date, PrimitiveDateTime, Time};

pub fn get_date_now() -> PrimitiveDateTime {
    let local_now = chrono::offset::Local::now();
    PrimitiveDateTime::new(
        Date::from_ordinal_date(local_now.year(), local_now.ordinal() as u16).unwrap(),
        Time::from_hms(
            local_now.hour() as u8,
            local_now.minute() as u8,
            local_now.second() as u8,
        )
        .unwrap(),
    )
}

pub fn get_utc_date_now() -> PrimitiveDateTime {
    let local_now = chrono::offset::Utc::now();
    PrimitiveDateTime::new(
        Date::from_ordinal_date(local_now.year(), local_now.ordinal() as u16).unwrap(),
        Time::from_hms(
            local_now.hour() as u8,
            local_now.minute() as u8,
            local_now.second() as u8,
        )
        .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use crate::*;

    use super::*;

    #[test]
    fn can_create_wp_post() {
        let p = WpPost::new(1);
        assert_eq!(p.post_author, 1);
        assert_eq!(p.post_status, PostStatus::Draft);
    }

    #[test]
    fn can_get_date_for_now() {
        let date = get_date_now();
        let now = chrono::Local::now();
        assert_eq!(date.day(), now.day() as u8);
        assert_eq!(date.year(), now.year());
    }

    #[test]
    fn can_get_date_for_now_utc() {
        let date = get_utc_date_now();
        let now = chrono::Utc::now();
        assert_eq!(date.day(), now.day() as u8);
        assert_eq!(date.year(), now.year());
    }
}
