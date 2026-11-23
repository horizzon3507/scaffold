//! The `flask` preset: Flask web app: app.py, requirements, gitignore

use crate::preset::Preset;
use crate::template::Template;

/// The `flask` preset.
pub const PRESET: Preset = Preset {
    name: "flask",
    description: "Flask web app: app.py, requirements, gitignore",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("app.py", include_str!("../../templates/flask/app.py")),
        Template::new(
            "requirements.txt",
            include_str!("../../templates/flask/requirements.txt"),
        ),
        Template::new(
            ".gitignore",
            include_str!("../../templates/flask/.gitignore"),
        ),
        Template::new("README.md", include_str!("../../templates/flask/README.md")),
    ]
}
