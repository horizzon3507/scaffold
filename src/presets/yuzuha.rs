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
    ]
}
