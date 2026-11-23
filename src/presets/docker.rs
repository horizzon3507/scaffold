//! The `docker` preset: Docker app: multi-stage Dockerfile, compose

use crate::preset::Preset;
use crate::template::Template;

/// The `docker` preset.
pub const PRESET: Preset = Preset {
    name: "docker",
    description: "Docker app: multi-stage Dockerfile, compose",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("Dockerfile", include_str!("../../templates/docker/Dockerfile")),
        Template::new(
            ".dockerignore",
            include_str!("../../templates/docker/.dockerignore"),
        ),
        Template::new(
            "compose.yaml",
            include_str!("../../templates/docker/compose.yaml"),
        ),
        Template::new("README.md", include_str!("../../templates/docker/README.md")),
    ]
}
