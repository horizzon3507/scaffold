//! The `python` preset: Python CLI: pyproject, src layout, smoke test

use crate::preset::Preset;
use crate::template::Template;

/// The `python` preset.
pub const PRESET: Preset = Preset {
    name: "python",
    description: "Python CLI: pyproject, src layout, smoke test",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new(
            "pyproject.toml",
            include_str!("../../templates/python/pyproject.toml"),
        ),
        Template::new(
            "src/{{name_snake}}/__init__.py",
            include_str!("../../templates/python/src/{{name_snake}}/__init__.py"),
        ),
        Template::new(
            "src/{{name_snake}}/__main__.py",
            include_str!("../../templates/python/src/{{name_snake}}/__main__.py"),
        ),
        Template::new(
            "tests/test_smoke.py",
            include_str!("../../templates/python/tests/test_smoke.py"),
        ),
        Template::new(
            "README.md",
            include_str!("../../templates/python/README.md"),
        ),
        Template::new(
            ".gitignore",
            include_str!("../../templates/python/.gitignore"),
        ),
        Template::new("LICENSE", include_str!("../../templates/python/LICENSE")),
    ]
}
