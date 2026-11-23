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
    ]
}
