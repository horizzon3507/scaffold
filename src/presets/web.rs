//! The `web` preset: Static web app: html, css, js, package.json

use crate::preset::Preset;
use crate::template::Template;

/// The `web` preset.
pub const PRESET: Preset = Preset {
    name: "web",
    description: "Static web app: html, css, js, package.json",
    templates: templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("index.html", include_str!("../../templates/web/index.html")),
        Template::new("style.css", include_str!("../../templates/web/style.css")),
        Template::new("app.js", include_str!("../../templates/web/app.js")),
        Template::new("package.json", include_str!("../../templates/web/package.json")),
        Template::new("README.md", include_str!("../../templates/web/README.md")),
    ]
}
