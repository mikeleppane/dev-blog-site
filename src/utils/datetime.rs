use chrono::{DateTime, Utc};

#[must_use]
pub fn format_date(date: &DateTime<Utc>) -> String {
    date.format("%B %d, %Y").to_string()
}
