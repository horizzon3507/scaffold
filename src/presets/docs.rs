//! The `docs` preset: Docs-only project: readme, guides, changelog

use crate::preset::Preset;
use crate::template::Template;

/// The `docs` preset.
pub const PRESET: Preset = Preset {
    name: "docs",
    description: "Docs-only project: readme, guides, changelog",
    templates: templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("README.md", include_str!("../../templates/docs/README.md")),
    ]
}
