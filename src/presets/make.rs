//! The `make` preset: C project driven by a Makefile

use crate::preset::Preset;
use crate::template::Template;

/// The `make` preset.
pub const PRESET: Preset = Preset {
    name: "make",
    description: "C project driven by a Makefile",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("Makefile", include_str!("../../templates/make/Makefile")),
        Template::new(
            "src/main.c",
            include_str!("../../templates/make/src/main.c"),
        ),
        Template::new(
            ".gitignore",
            include_str!("../../templates/make/.gitignore"),
        ),
        Template::new("README.md", include_str!("../../templates/make/README.md")),
    ]
}
