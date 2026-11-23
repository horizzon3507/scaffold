//! The `vue` preset: Vue app: vite config, App.vue, index

use crate::preset::Preset;
use crate::template::Template;

/// The `vue` preset.
pub const PRESET: Preset = Preset {
    name: "vue",
    description: "Vue app: vite config, App.vue, index",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new(
            "package.json",
            include_str!("../../templates/vue/package.json"),
        ),
        Template::new(
            "vite.config.js",
            include_str!("../../templates/vue/vite.config.js"),
        ),
        Template::new("index.html", include_str!("../../templates/vue/index.html")),
        Template::new("src/main.js", include_str!("../../templates/vue/src/main.js")),
        Template::new(
            "src/App.vue",
            include_str!("../../templates/vue/src/App.vue"),
        ),
        Template::new(".gitignore", include_str!("../../templates/vue/.gitignore")),
        Template::new("README.md", include_str!("../../templates/vue/README.md")),
    ]
}
