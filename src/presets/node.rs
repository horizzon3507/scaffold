//! The `node` preset: Node + TypeScript CLI: bin, tsconfig, entrypoint

use crate::preset::Preset;
use crate::template::Template;

/// The `node` preset.
pub const PRESET: Preset = Preset {
    name: "node",
    description: "Node + TypeScript CLI: bin, tsconfig, entrypoint",
    templates: templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("package.json", include_str!("../../templates/node/package.json")),
        Template::new("tsconfig.json", include_str!("../../templates/node/tsconfig.json")),
        Template::new("src/index.ts", include_str!("../../templates/node/src/index.ts")),
        Template::new("README.md", include_str!("../../templates/node/README.md")),
    ]
}
