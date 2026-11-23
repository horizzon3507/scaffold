//! The `express` preset: Express API: app.js, package.json

use crate::preset::Preset;
use crate::template::Template;

/// The `express` preset.
pub const PRESET: Preset = Preset {
    name: "express",
    description: "Express API: app.js, package.json",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("app.js", include_str!("../../templates/express/app.js")),
        Template::new(
            "package.json",
            include_str!("../../templates/express/package.json"),
        ),
        Template::new(
            ".gitignore",
            include_str!("../../templates/express/.gitignore"),
        ),
        Template::new(
            "README.md",
            include_str!("../../templates/express/README.md"),
        ),
    ]
}
