use wp_query_rs::*;

#[test]
fn year_month_day() {
    let params = ParamBuilder::new().year(2023).monthnum(1).day(1);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
}

#[test]
fn year_month_day_hour() {
    let params = ParamBuilder::new().year(2023).monthnum(1).day(1).hour(4);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
}

#[test]
fn year_month_day_hour_minute() {
    let params = ParamBuilder::new().year(2023).monthnum(1).day(1).hour(4).minute(23);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
}

#[test]
fn year_month_day_hour_minute_second() {
    let params = ParamBuilder::new().year(2023).monthnum(1).day(1).hour(4).minute(23).second(12);

    let posts = WP_Query::new(params.params()).expect("SqlFailed");
}
