//! Optional user configuration at `~/.config/scaffold/config`.
//!
//! The format is deliberately simple: `key = value` lines, `#` comments,
//! blank lines ignored. Supported keys: `preset`, `author`, `vcs`.

/// Settings read from the config file. Everything is optional.
#[derive(Default)]
pub struct Config {
    /// Default preset when `-p/--preset` is not given.
    pub default_preset: Option<String>,
    /// Default author recorded in generated metadata.
    pub author: Option<String>,
    /// Default version control: "git" or "none".
    pub vcs: Option<String>,
}

impl Config {
    /// Parse config text. Unknown keys and bad lines are ignored, so a
    /// newer config never breaks an older binary.
    pub fn parse(text: &str) -> Config {
        let mut cfg = Config::default();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let value = value.trim().trim_matches('"').to_string();
            if value.is_empty() {
                continue;
            }
            match key.trim() {
                "preset" => cfg.default_preset = Some(value),
                "author" => cfg.author = Some(value),
                "vcs" => cfg.vcs = Some(value),
                _ => {}
            }
        }
        cfg
    }
}

/// The config file path: $SCAFFOLD_CONFIG, else
/// $XDG_CONFIG_HOME/scaffold/config, else ~/.config/scaffold/config.
pub fn path() -> Option<std::path::PathBuf> {
    if let Some(p) = std::env::var_os("SCAFFOLD_CONFIG") {
        return Some(std::path::PathBuf::from(p));
    }
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config")))?;
    Some(base.join("scaffold").join("config"))
}

/// Load the user's config; missing or unreadable files give defaults.
pub fn load() -> Config {
    path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|text| Config::parse(&text))
        .unwrap_or_default()
}
