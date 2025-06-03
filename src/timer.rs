use chrono::offset::MappedLocalTime;
use chrono::{self, Datelike, Weekday};

#[derive(Debug)]
pub struct UnspotDateData {
    pub start: i64,
    pub end: i64,
    pub date: String,
}

impl UnspotDateData {
    pub fn from_today() -> UnspotDateData {
        let today = chrono::Local::now();

        let add_days = match today.weekday() {
            Weekday::Fri => 3,
            Weekday::Sat => 2,
            _ => 1,
        };

        let target_date = today + chrono::Duration::days(add_days);

        let start = MappedLocalTime::Single(
            chrono::NaiveDate::from_ymd_opt(
                target_date.year(),
                target_date.month(),
                target_date.day(),
            )
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap(),
        )
        .unwrap()
        .and_utc()
        .timestamp();

        let end = MappedLocalTime::Single(
            chrono::NaiveDate::from_ymd_opt(
                target_date.year(),
                target_date.month(),
                target_date.day(),
            )
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap(),
        )
        .unwrap()
        .and_utc()
        .timestamp();

        UnspotDateData {
            start: start,
            end: end,
            date: target_date.format("%Y%m%d").to_string(),
        }
    }
}
