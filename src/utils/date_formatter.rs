use chrono::{DateTime, Utc};

/// Formats a UTC datetime into a human-readable format
#[must_use]
pub fn format_date_readable(date: DateTime<Utc>) -> String {
    date.format("%B %d, %Y").to_string()
}

/// Formats a UTC datetime into a short format
#[must_use]
pub fn format_date_short(date: DateTime<Utc>) -> String {
    date.format("%b %d, %Y").to_string()
}

/// Formats a UTC datetime into ISO format
#[must_use]
pub fn format_date_iso(date: DateTime<Utc>) -> String {
    date.to_rfc3339()
}

/// Returns a relative time string (e.g., "2 days ago")
#[must_use]
pub fn format_date_relative(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(date);

    if duration.num_days() > 30 {
        format_date_readable(date)
    } else if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else {
        "Just now".to_string()
    }
}
