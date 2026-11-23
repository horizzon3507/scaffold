//! Variable rendering for template contents and paths.

use std::process::Command;

use crate::name;

/// Values substituted into `{{variable}}` slots.
pub struct Context {
    /// Project name as given, e.g. `my-app`.
    pub name: String,
    /// Title-cased name, e.g. `My App`.
    pub title: String,
    /// Author recorded in metadata, e.g. `horizzon3507`.
    pub author: String,
    /// Current year, e.g. `2026`.
    pub year: String,
    /// Today's date as DD/MM/YYYY.
    pub date: String,
}

impl Context {
    /// Build a context for a project.
    pub fn new(name: &str, author: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            title: name::title_case(name),
            author: author.map(str::to_string).unwrap_or_else(detect_author),
            year: year(),
            date: today(),
        }
    }
}

/// Days since the Unix epoch to a `(year, month, day)` civil date.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    // Howard Hinnant's civil_from_days algorithm.
    let z = days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn now_ymd() -> (i64, u32, u32) {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    civil_from_days((secs / 86_400) as i64)
}

/// Today's date as DD/MM/YYYY.
pub fn today() -> String {
    let (y, m, d) = now_ymd();
    format!("{d:02}/{m:02}/{y}")
}

/// The current year.
pub fn year() -> String {
    now_ymd().0.to_string()
}

impl Context {
    /// Replace every `{{variable}}` slot in `text`.
    pub fn render(&self, text: &str) -> String {
        text.replace("{{name}}", &self.name)
            .replace("{{Name}}", &self.title)
            .replace("{{author}}", &self.author)
            .replace("{{year}}", &self.year)
            .replace("{{date}}", &self.date)
    }
}

impl Context {
    /// Render a template path and re-validate the result.
    pub fn render_path(&self, path: &str) -> Result<String, crate::error::Invalid> {
        let rendered = self.render(path);
        crate::template::validate_path(&rendered)?;
        Ok(rendered)
    }
}

/// Author name from `git config user.name`, or "you" when unset.
pub fn detect_author() -> String {
    Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "you".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> Context {
        Context::new("my-app", Some("hory"))
    }

    #[test]
    fn renders_all_variables() {
        let out = ctx().render("{{name}} {{Name}} {{author}} {{year}} {{date}}");
        assert!(out.starts_with("my-app My App hory "));
        assert!(!out.contains("{{"));
    }

    #[test]
    fn renders_paths_and_revalidates() {
        assert_eq!(
            ctx().render_path("src/{{name}}/mod.rs").unwrap(),
            "src/my-app/mod.rs"
        );
    }

    #[test]
    fn civil_dates_match_known_days() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_782), (2024, 2, 29));
    }
}
