//! Error type for validation failures.

use std::error::Error;
use std::fmt;

/// A validation failure with a hint for the user.
#[derive(Debug)]
pub struct Invalid {
    /// What failed validation, e.g. "project name".
    pub what: &'static str,
    /// Why it failed, in plain words.
    pub why: String,
}

impl Invalid {
    /// Build a new validation error.
    pub fn new(what: &'static str, why: impl Into<String>) -> Self {
        Self {
            what,
            why: why.into(),
        }
    }
}

impl fmt::Display for Invalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}: {}", self.what, self.why)
    }
}

impl Error for Invalid {}
