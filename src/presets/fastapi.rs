//! The `fastapi` preset: FastAPI service: main.py, requirements

use crate::preset::Preset;
use crate::template::Template;

/// The `fastapi` preset.
pub const PRESET: Preset = Preset {
    name: "fastapi",
    description: "FastAPI service: main.py, requirements",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("main.py", include_str!("../../templates/fastapi/main.py")),
        Template::new(
            "requirements.txt",
            include_str!("../../templates/fastapi/requirements.txt"),
        ),
        Template::new(
            ".gitignore",
            include_str!("../../templates/fastapi/.gitignore"),
        ),
        Template::new(
            "README.md",
            include_str!("../../templates/fastapi/README.md"),
        ),
    ]
}
