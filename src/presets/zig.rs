//! The `zig` preset: Zig CLI: build.zig, main, gitignore

use crate::preset::Preset;
use crate::template::Template;

/// The `zig` preset.
pub const PRESET: Preset = Preset {
    name: "zig",
    description: "Zig CLI: build.zig, main, gitignore",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("build.zig", include_str!("../../templates/zig/build.zig")),
        Template::new(
            "src/main.zig",
            include_str!("../../templates/zig/src/main.zig"),
        ),
        Template::new(".gitignore", include_str!("../../templates/zig/.gitignore")),
        Template::new("README.md", include_str!("../../templates/zig/README.md")),
    ]
}
