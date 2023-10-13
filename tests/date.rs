use wp_query_rs::*;

#[cfg(feature = "query_sync")]
#[test]
fn year_month_day() {
    let params = ParamBuilder::new().year(2023).monthnum(1).day(1);

    let posts = WP_Query::new(params).expect("SqlFailed");
}

#[cfg(feature = "query_sync")]
#[test]
fn year_month_day_hour() {
    let params = ParamBuilder::new().year(2023).monthnum(1).day(1).hour(4);

    let posts = WP_Query::new(params).expect("SqlFailed");
}

#[cfg(feature = "query_sync")]
#[test]
fn year_month_day_hour_minute() {
    let params = ParamBuilder::new()
        .year(2023)
        .monthnum(1)
        .day(1)
        .hour(4)
        .minute(23);

    let posts = WP_Query::new(params).expect("SqlFailed");
}

#[cfg(feature = "query_sync")]
#[test]
fn year_month_day_hour_minute_second() {
    let params = ParamBuilder::new()
        .year(2023)
        .monthnum(1)
        .day(1)
        .hour(4)
        .minute(23)
        .second(12);

    let posts = WP_Query::new(params).expect("SqlFailed");
}

#[cfg(feature = "query_sync")]
#[test]
fn date_query() {
    let params = ParamBuilder::new().date_query(
        DateQuery::new()
            .after(DateQueryAfterBefore {
                year: 2022,
                month: 2,
                day: 1,
            })
            .before(DateQueryAfterBefore {
                year: 2023,
                month: 8,
                day: 23,
            })
            .inclusive(true),
    ).orderby(WpOrderBy::Date);

    let posts = WP_Query::new(params).expect("SqlFailed");
}
