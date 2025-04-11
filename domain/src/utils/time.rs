use chrono::{DateTime, Utc};

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
        format!("{years} y")
    } else if months > 0 {
        format!("{months} mo")
    } else if days > 0 {
        format!("{days} d")
    } else if hours > 0 {
        format!("{hours} h")
    } else if minutes > 0 {
        format!("{minutes} m")
    } else {
        format!("{seconds} s")
    }
}
