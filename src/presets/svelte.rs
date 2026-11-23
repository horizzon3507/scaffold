//! The `svelte` preset: Svelte app: vite config, App.svelte, index

use crate::preset::Preset;
use crate::template::Template;

/// The `svelte` preset.
pub const PRESET: Preset = Preset {
    name: "svelte",
    description: "Svelte app: vite config, App.svelte, index",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new(
            "package.json",
            include_str!("../../templates/svelte/package.json"),
        ),
        Template::new(
            "vite.config.js",
            include_str!("../../templates/svelte/vite.config.js"),
        ),
        Template::new("index.html", include_str!("../../templates/svelte/index.html")),
        Template::new("src/main.js", include_str!("../../templates/svelte/src/main.js")),
        Template::new(
            "src/App.svelte",
            include_str!("../../templates/svelte/src/App.svelte"),
        ),
        Template::new(".gitignore", include_str!("../../templates/svelte/.gitignore")),
        Template::new("README.md", include_str!("../../templates/svelte/README.md")),
    ]
}
