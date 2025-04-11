use chrono::{DateTime, Utc};

pub fn format_timestamp(timestamp: &str, format: Option<&str>) -> String {
    let format = format.unwrap_or("%Y-%m-%d %H:%M:%S %Z");
    timestamp.parse::<DateTime<Utc>>().map_or_else(
        |_| "Invalid timestamp".to_string(),
        |parsed_timestamp| parsed_timestamp.format(format).to_string(),
    )
}
