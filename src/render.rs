//! Variable rendering for template contents and paths.

use crate::name;

/// Values substituted into `{{variable}}` slots.
pub struct Context {
    /// Project name as given, e.g. `my-app`.
    pub name: String,
    /// Title-cased name, e.g. `My App`.
    pub title: String,
    /// Author recorded in metadata, e.g. `horizzon3507`.
    pub author: String,
}

impl Context {
    /// Build a context for a project.
    pub fn new(name: &str, author: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            title: name::title_case(name),
            author: author.unwrap_or("you").to_string(),
        }
    }
}
