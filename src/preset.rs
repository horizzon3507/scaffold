//! Preset model and registry.

use crate::color;
use crate::error::AppResult as Result;

use crate::presets;
use crate::template::Template;

/// A named bundle of templates.
pub struct Preset {
    /// Short name used on the command line, e.g. `hory`.
    pub name: &'static str,
    /// One-line description for `scaffold list`.
    pub description: &'static str,
    /// Files the preset writes.
    pub templates: fn() -> Vec<Template>,
}

/// Every preset, in display order. `hory` stays first: it is the default.
pub fn all() -> Vec<Preset> {
    vec![
        presets::hory::PRESET,
        presets::rust::PRESET,
        presets::web::PRESET,
        presets::yuzuha::PRESET,
        presets::python::PRESET,
        presets::node::PRESET,
        presets::go::PRESET,
        presets::docs::PRESET,
        presets::latex::PRESET,
        presets::zig::PRESET,
        presets::deno::PRESET,
        presets::flask::PRESET,
        presets::docker::PRESET,
        presets::astro::PRESET,
        presets::make::PRESET,
        presets::terraform::PRESET,
    ]
}

/// Look a preset up by name.
pub fn find(name: &str) -> Option<Preset> {
    all().into_iter().find(|p| p.name == name)
}

/// Look a preset up, or fail listing what exists.
pub fn find_or_error(name: &str) -> Result<Preset> {
    match find(name) {
        Some(p) => Ok(p),
        None => {
            let known = all().iter().map(|p| p.name).collect::<Vec<_>>().join(", ");
            Err(format!("unknown preset '{name}' (available: {known})").into())
        }
    }
}

/// The list as one JSON document.
fn list_json() -> String {
    let items: Vec<String> = all()
        .iter()
        .map(|p| {
            format!(
                "{{\"name\":{},\"description\":{},\"default\":{}}}",
                crate::json::string(p.name),
                crate::json::string(p.description),
                p.name == "hory"
            )
        })
        .collect();
    format!("{{\"presets\":{}}}", crate::json::array(&items))
}

/// Print the presets for `scaffold list`.
pub fn run_list(args: &crate::cli::ListArgs) -> Result<()> {
    if args.json {
        crate::out!("{}", list_json());
        return Ok(());
    }
    for p in all() {
        let name = color::cyan(p.name);
        let default = if p.name == "hory" {
            color::dim(" (default)")
        } else {
            String::new()
        };
        if args.verbose {
            crate::out!("{name:<17} {}{}", p.description, default);
        } else {
            crate::out!("{name}{default}");
        }
    }
    Ok(())
}

/// One preset's file plan as JSON.
fn describe_json(p: &Preset, templates: &[Template]) -> String {
    let files: Vec<String> = templates
        .iter()
        .map(|t| {
            format!(
                "{{\"path\":{},\"bytes\":{}}}",
                crate::json::string(t.path),
                t.contents.len()
            )
        })
        .collect();
    format!(
        "{{\"preset\":{},\"description\":{},\"files\":{}}}",
        crate::json::string(p.name),
        crate::json::string(p.description),
        crate::json::array(&files)
    )
}

/// Show what a preset generates.
pub fn run_describe(args: &crate::cli::DescribeArgs) -> Result<()> {
    let p = find_or_error(&args.preset)?;
    if args.tree {
        let templates = (p.templates)();
        for t in &templates {
            t.validate()?;
        }
        let paths: Vec<String> = templates.iter().map(|t| t.path.to_string()).collect();
        crate::out!("{}", color::bold(p.name));
        crate::out!("{}", crate::tree::render(&paths).trim_end());
        return Ok(());
    }
    if args.json {
        let mut templates = (p.templates)();
        templates.sort_by(|a, b| a.path.cmp(b.path));
        for t in &templates {
            t.validate()?;
        }
        crate::out!("{}", describe_json(&p, &templates));
        return Ok(());
    }
    crate::out!("{} - {}", color::bold(p.name), color::dim(p.description));
    let mut templates = (p.templates)();
    templates.sort_by(|a, b| a.path.cmp(b.path));
    for t in &templates {
        t.validate()?;
        let size = color::dim(&format!("{:>6} bytes", t.contents.len()));
        crate::out!("  {:<53} {size}", t.path);
    }
    Ok(())
}
