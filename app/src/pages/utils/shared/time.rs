use chrono::{DateTime, Utc};

pub fn format_timestamp(timestamp: &str, format: Option<&str>) -> String {
    let format = format.unwrap_or("%Y-%m-%d %H:%M:%S %Z");
    if let Ok(parsed_timestamp) = timestamp.parse::<DateTime<Utc>>() {
        parsed_timestamp.format(format).to_string()
    } else {
        "Invalid timestamp".to_string()
    }
}

#[allow(dead_code)]
pub fn time_until_now(timestamp: &str) -> String {
    if timestamp.is_empty() { return "-".to_string(); }

    let timestamp: DateTime<Utc> = timestamp.parse().expect("Failed to parse timestamp");
    let now = Utc::now();
    let duration = now.signed_duration_since(timestamp);

    let total_seconds = duration.num_seconds();
    let days = total_seconds / 86400;
    let months = days / 30;
    let years = months / 12;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if years > 0 {
        format!("{} y", years)
    } else if months > 0 {
        format!("{} mo", months)
    } else if days > 0 {
        format!("{} d", days)
    } else if hours > 0 {
        format!("{} h", hours)
    } else if minutes > 0 {
        format!("{} m", minutes)
    } else {
        format!("{} s", seconds)
    }
}
