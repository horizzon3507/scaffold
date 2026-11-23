//! The `python` preset: Python CLI: pyproject, src layout, smoke test

use crate::preset::Preset;
use crate::template::Template;

/// The `python` preset.
pub const PRESET: Preset = Preset {
    name: "python",
    description: "Python CLI: pyproject, src layout, smoke test",
    templates: templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("pyproject.toml", include_str!("../../templates/python/pyproject.toml")),
    ]
}
