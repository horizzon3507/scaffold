//! The `go` preset: Go CLI: go.mod, main.go, Makefile

use crate::preset::Preset;
use crate::template::Template;

/// The `go` preset.
pub const PRESET: Preset = Preset {
    name: "go",
    description: "Go CLI: go.mod, main.go, Makefile",
    templates: templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("go.mod", include_str!("../../templates/go/go.mod")),
    ]
}
