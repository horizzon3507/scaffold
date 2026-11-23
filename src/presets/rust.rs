//! The `rust` preset: Minimal Rust CLI: manifest, main, license, changelog

use crate::preset::Preset;
use crate::template::Template;

/// The `rust` preset.
pub const PRESET: Preset = Preset {
    name: "rust",
    description: "Minimal Rust CLI: manifest, main, license, changelog",
    templates: templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("Cargo.toml", include_str!("../../templates/rust/Cargo.toml")),
        Template::new("src/main.rs", include_str!("../../templates/rust/src/main.rs")),
    ]
}
