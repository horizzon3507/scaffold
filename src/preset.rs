//! Preset model and registry.

use anyhow::bail;
use anyhow::Result;

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
    vec![]
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
            bail!("unknown preset '{name}' (available: {known})")
        }
    }
}


/// Print the presets for `scaffold list`.
pub fn run_list(_args: &crate::args::ListArgs) -> Result<()> {
    for p in all() {
        println!("{}", p.name);
    }
    Ok(())
}
