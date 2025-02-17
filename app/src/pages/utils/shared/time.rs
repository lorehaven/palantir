use chrono::{DateTime, Utc};

#[allow(dead_code)]
pub fn time_until_now(timestamp: &str) -> String {
    let timestamp: DateTime<Utc> = timestamp.parse().expect("Failed to parse timestamp");
    let now = Utc::now();
    let duration = now.signed_duration_since(timestamp);

    let total_seconds = duration.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{} h", hours)
    } else if minutes > 0 {
        format!("{} m", minutes)
    } else {
        format!("{} s", seconds)
    }
}
