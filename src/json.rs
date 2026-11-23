//! Minimal JSON writing, no dependencies: enough for list and describe.

/// Escape a string for JSON.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// A quoted JSON string.
pub fn string(text: &str) -> String {
    format!("\"{}\"", escape(text))
}

/// Join already-rendered values into a JSON array.
pub fn array(items: &[String]) -> String {
    format!("[{}]", items.join(","))
}
