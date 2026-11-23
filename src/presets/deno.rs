//! The `deno` preset: Deno TypeScript CLI: main.ts, deno.json

use crate::preset::Preset;
use crate::template::Template;

/// The `deno` preset.
pub const PRESET: Preset = Preset {
    name: "deno",
    description: "Deno TypeScript CLI: main.ts, deno.json",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("main.ts", include_str!("../../templates/deno/main.ts")),
        Template::new("deno.json", include_str!("../../templates/deno/deno.json")),
        Template::new(".gitignore", include_str!("../../templates/deno/.gitignore")),
        Template::new("README.md", include_str!("../../templates/deno/README.md")),
    ]
}
