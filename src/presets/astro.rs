//! The `astro` preset: Astro site: config, index page, package.json

use crate::preset::Preset;
use crate::template::Template;

/// The `astro` preset.
pub const PRESET: Preset = Preset {
    name: "astro",
    description: "Astro site: config, index page, package.json",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new(
            "package.json",
            include_str!("../../templates/astro/package.json"),
        ),
        Template::new(
            "astro.config.mjs",
            include_str!("../../templates/astro/astro.config.mjs"),
        ),
        Template::new(
            "src/pages/index.astro",
            include_str!("../../templates/astro/src/pages/index.astro"),
        ),
        Template::new(
            ".gitignore",
            include_str!("../../templates/astro/.gitignore"),
        ),
        Template::new("README.md", include_str!("../../templates/astro/README.md")),
    ]
}
