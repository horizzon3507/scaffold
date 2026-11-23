//! Tiny ANSI styling with no dependencies.
//!
//! Colors are on when stdout is a terminal, off when the user passes
//! `--no-color` or sets the `NO_COLOR` environment variable.

use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(true);

/// Decide once at startup. `disabled_by_flag` is the `--no-color` flag.
pub fn init(disabled_by_flag: bool) {
    let on = !disabled_by_flag
        && std::env::var_os("NO_COLOR").is_none()
        && std::io::stdout().is_terminal();
    ENABLED.store(on, Ordering::Relaxed);
}

/// Whether styling is active right now.
pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// Override the decision; used by tests.
pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}

fn wrap(code: &str, text: &str) -> String {
    if is_enabled() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

/// Bold text, for headings and names.
pub fn bold(text: &str) -> String {
    wrap("1", text)
}

/// Green text, for success lines.
pub fn green(text: &str) -> String {
    wrap("32", text)
}

/// Red text, for errors.
pub fn red(text: &str) -> String {
    wrap("31", text)
}

/// Yellow text, for warnings.
pub fn yellow(text: &str) -> String {
    wrap("33", text)
}

/// Cyan text, for names in listings.
pub fn cyan(text: &str) -> String {
    wrap("36", text)
}

/// Dimmed text, for secondary detail.
pub fn dim(text: &str) -> String {
    wrap("2", text)
}
