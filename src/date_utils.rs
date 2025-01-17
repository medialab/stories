use chrono::NaiveDateTime;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Tz, UTC};
use dateparser::parse_with_timezone;
use std::error::Error;

const LONG_DATE_FORMAT: &str = "%a %b %d %H:%M:%S +0000 %Y";
const SHORT_DATE_FORMAT: &str = "%a %b %d %H:%M:%S";
const REGULAR_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const DAY_DATE_FORMAT: &str = "%Y-%m-%d";

fn date_from_string(date_str: &str, date_format: &str) -> Result<NaiveDateTime, Box<dyn Error>> {
    let datetime =
        NaiveDateTime::parse_from_str(date_str, date_format).or(Err("Unknown date format!"))?;
    Ok(datetime)
}

fn date_from_timestamp(
    timestamp_str: &str,
    timezone: &Tz,
) -> Result<NaiveDateTime, Box<dyn Error>> {
    let datetime = NaiveDateTime::from_timestamp_opt(
        timestamp_str
            .parse::<i64>()
            .or(Err("Unknown date format!"))?,
        0,
    ).unwrap();
    let utc_datetime = UTC.from_local_datetime(&datetime).unwrap();
    Ok(utc_datetime.with_timezone(timezone).naive_local())
}

pub fn inferred_date(date_cell: &str, tz: &Tz) -> Result<NaiveDateTime, Box<dyn Error>> {
    // Inferring date format from the string...
    let datetime = if date_cell.contains('+') {
        date_from_string(date_cell, LONG_DATE_FORMAT)?
    } else if date_cell.chars().any(|c| c.is_ascii_alphabetic()) {
        date_from_string(date_cell, SHORT_DATE_FORMAT)?
    } else if date_cell.contains('-') {
        if date_cell.len() == 10 {
            date_from_string(date_cell, DAY_DATE_FORMAT)?
        } else {
            date_from_string(date_cell, REGULAR_DATE_FORMAT)?
        }
    } else {
        date_from_timestamp(date_cell, &tz)?
    };
    Ok(datetime)
}

pub fn parse_timezone(tz: String) -> Result<Tz, String> {
    match tz.parse::<Tz>() {
        Ok(timezone) => Ok(timezone),
        _ => Err(format!("{} is not a valid timezone", tz)),
    }
}

pub fn parse_date(date: &str, tz: Tz) -> Result<DateTime<Utc>, String> {
    match parse_with_timezone(&date, &tz) {
        Ok(time) => Ok(time),
        _ => Err(format!("Time format could not be inferred for {}", date)),
    }
}
