#[must_use]
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let mut truncated = text.chars().take(max_length).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}
