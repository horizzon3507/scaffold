//! A single file a preset writes out.

/// One file in a preset: where it lands and what it contains.
pub struct Template {
    /// Path relative to the project root, forward slashes.
    pub path: &'static str,
    /// Raw contents, before variable rendering.
    pub contents: &'static str,
}

impl Template {
    /// Define a template entry.
    pub const fn new(path: &'static str, contents: &'static str) -> Self {
        Self { path, contents }
    }
}
