//! The `go` preset: Go CLI: go.mod, main.go, Makefile

use crate::preset::Preset;
use crate::template::Template;

/// The `go` preset.
pub const PRESET: Preset = Preset {
    name: "go",
    description: "Go CLI: go.mod, main.go, Makefile",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("go.mod", include_str!("../../templates/go/go.mod")),
        Template::new("main.go", include_str!("../../templates/go/main.go")),
        Template::new("Makefile", include_str!("../../templates/go/Makefile")),
        Template::new("README.md", include_str!("../../templates/go/README.md")),
        Template::new(".gitignore", include_str!("../../templates/go/.gitignore")),
    ]
}
