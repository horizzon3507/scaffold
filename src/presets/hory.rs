//! The `hory` preset: Hory-style Rust CLI: clap, CI, changelog, readme, license

use crate::preset::Preset;
use crate::template::Template;

/// The `hory` preset.
pub const PRESET: Preset = Preset {
    name: "hory",
    description: "Hory-style Rust CLI: clap, CI, changelog, readme, license",
    templates: templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("Cargo.toml", include_str!("../../templates/hory/Cargo.toml")),
        Template::new("src/main.rs", include_str!("../../templates/hory/src/main.rs")),
    ]
}
