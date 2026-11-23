//! The `terraform` preset: Terraform module: main, variables, outputs

use crate::preset::Preset;
use crate::template::Template;

/// The `terraform` preset.
pub const PRESET: Preset = Preset {
    name: "terraform",
    description: "Terraform module: main, variables, outputs",
    templates,
};

fn templates() -> Vec<Template> {
    vec![
        Template::new("main.tf", include_str!("../../templates/terraform/main.tf")),
        Template::new(
            "variables.tf",
            include_str!("../../templates/terraform/variables.tf"),
        ),
        Template::new(
            "outputs.tf",
            include_str!("../../templates/terraform/outputs.tf"),
        ),
        Template::new(
            ".gitignore",
            include_str!("../../templates/terraform/.gitignore"),
        ),
        Template::new(
            "README.md",
            include_str!("../../templates/terraform/README.md"),
        ),
    ]
}
