use chrono::DateTime;
use prost_types::Timestamp;

pub fn format_timestamp(timestamp: Option<Timestamp>) -> String {
    match timestamp {
        Some(ts) => {
            match DateTime::from_timestamp(ts.seconds, ts.nanos as u32) {
                Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
                None => "Invalid Date".to_string(),
            }
        }
        None => "Unknown".to_string(),
    }
}