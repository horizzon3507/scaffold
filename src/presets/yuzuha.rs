//! The `yuzuha` preset: Yuzuha bot for Fluxer: tsconfig, ping/eco commands, env

use crate::preset::Preset;
use crate::template::Template;

/// The `yuzuha` preset.
pub const PRESET: Preset = Preset {
    name: "yuzuha",
    description: "Yuzuha bot for Fluxer: tsconfig, ping/eco commands, env",
    templates: templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("package.json", include_str!("../../templates/yuzuha/package.json")),
        Template::new("tsconfig.json", include_str!("../../templates/yuzuha/tsconfig.json")),
        Template::new("commands/ping.ts", include_str!("../../templates/yuzuha/commands/ping.ts")),
        Template::new("commands/eco.ts", include_str!("../../templates/yuzuha/commands/eco.ts")),
        Template::new(".env.example", include_str!("../../templates/yuzuha/.env.example")),
        Template::new(".gitignore", include_str!("../../templates/yuzuha/.gitignore")),
        Template::new("README.md", include_str!("../../templates/yuzuha/README.md")),
    ]
}
