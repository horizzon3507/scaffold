//! The `latex` preset: LaTeX paper: main.tex, Makefile, gitignore

use crate::preset::Preset;
use crate::template::Template;

/// The `latex` preset.
pub const PRESET: Preset = Preset {
    name: "latex",
    description: "LaTeX paper: main.tex, Makefile, gitignore",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("main.tex", include_str!("../../templates/latex/main.tex")),
        Template::new("Makefile", include_str!("../../templates/latex/Makefile")),
        Template::new(".gitignore", include_str!("../../templates/latex/.gitignore")),
        Template::new("README.md", include_str!("../../templates/latex/README.md")),
    ]
}
