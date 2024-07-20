pub fn time_after_duration(duration: std::time::Duration) -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::from_std(duration).unwrap())
        .unwrap_or_else(chrono::Utc::now)
}

/// Format a date into a discord relative-time timestamp.
pub fn format_date_ago(date: chrono::DateTime<chrono::Utc>) -> String {
    format!("<t:{}:R>", date.timestamp())
}