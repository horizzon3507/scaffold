//! A single file a preset writes out.

use crate::error::Invalid;

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

    /// Reject paths that could escape the project root.
    pub fn validate(&self) -> Result<(), Invalid> {
        let path = self.path;
        if path.starts_with('/') || path.starts_with('\\') {
            return Err(Invalid::new("template path", "it must be relative"));
        }
        if path.split('/').any(|part| part == "..") {
            return Err(Invalid::new("template path", "it must not contain '..'"));
        }
        if path.contains(':') {
            return Err(Invalid::new("template path", "it must not contain ':'"));
        }
        Ok(())
    }
}
