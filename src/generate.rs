//! Planning and writing generated projects.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::AppResult as Result;

use crate::cli::{NewArgs, Vcs};
use crate::color;
use crate::preset;
use crate::render;

/// One file ready to land on disk.
struct PlannedFile {
    dest: PathBuf,
    contents: String,
}

/// A resolved generation job.
struct Plan {
    root: PathBuf,
    files: Vec<PlannedFile>,
}

impl Plan {
    /// Resolve templates, variables and the target directory.
    fn build(args: &NewArgs, preset_name: &str, author: Option<&str>) -> Result<Self> {
        crate::name::validate(&args.name)?;
        let p = preset::find_or_error(preset_name)?;
        let ctx = render::Context::new(&args.name, author);
        let root = args
            .dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(&args.name));
        let mut files = Vec::new();
        for t in (p.templates)() {
            t.validate()?;
            files.push(PlannedFile {
                dest: root.join(ctx.render_path(t.path)?),
                contents: ctx.render(t.contents),
            });
        }
        Ok(Self { root, files })
    }

    /// Write every file, honoring `force`.
    fn execute(&self, force: bool) -> Result<()> {
        let clashes: Vec<_> = self.files.iter().filter(|f| f.dest.exists()).collect();
        if !clashes.is_empty() && !force {
            let names = clashes
                .iter()
                .map(|f| f.dest.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(format!(
                "{} path(s) already exist, use --force to overwrite: {}",
                clashes.len(),
                names
            )
            .into());
        }
        for f in &self.files {
            if let Some(parent) = f.dest.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("creating {}: {e}", parent.display()))?;
            }
            fs::write(&f.dest, &f.contents)
                .map_err(|e| format!("writing {}: {e}", f.dest.display()))?;
        }
        Ok(())
    }

    /// Print what would happen, without writing anything.
    fn print(&self) {
        crate::out!(
            "would create {} file(s) under {}",
            color::bold(&self.files.len().to_string()),
            self.root.display()
        );
        for f in &self.files {
            let size = color::dim(&format!("({} bytes)", f.contents.len()));
            crate::out!("  {} {size}", f.dest.display());
        }
    }
}

/// Run `scaffold new`.
pub fn run_new(args: &NewArgs) -> Result<()> {
    let cfg = crate::config::load();
    let preset_name = resolve_preset(args, &cfg)?;
    let author = args.author.clone().or(cfg.author);
    let vcs = match args.vcs.or_else(|| cfg.vcs.as_deref().and_then(parse_vcs)) {
        Some(v) => v,
        None => Vcs::Git,
    };
    let plan = Plan::build(args, &preset_name, author.as_deref())?;
    if args.dry_run {
        plan.print();
        return Ok(());
    }
    plan.execute(args.force)?;
    match vcs {
        Vcs::Git => init_git(&plan.root),
        Vcs::None => {}
    }
    print_next_steps(args, &plan, &preset_name);
    Ok(())
}

/// The preset for a run: the flag, then the config, then the picker on a
/// terminal, then the default.
fn resolve_preset(args: &NewArgs, cfg: &crate::config::Config) -> Result<String> {
    if let Some(p) = args.preset.clone().or_else(|| cfg.default_preset.clone()) {
        return Ok(p);
    }
    if args.no_input || !interactive() {
        return Ok("hory".to_string());
    }
    let presets = preset::all();
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    Ok(crate::pick::choose(
        &presets,
        "hory",
        &mut stdin,
        &mut stdout,
    )?)
}

/// True when both stdin and stdout are terminals.
fn interactive() -> bool {
    use std::io::IsTerminal;
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

/// Read the config's vcs string; an unknown value is ignored.
fn parse_vcs(value: &str) -> Option<Vcs> {
    match value {
        "git" => Some(Vcs::Git),
        "none" => Some(Vcs::None),
        _ => None,
    }
}

/// What to do with the fresh project.
fn print_next_steps(args: &NewArgs, plan: &Plan, preset_name: &str) {
    crate::out!(
        "{} {} in {}",
        color::green("created"),
        color::bold(&args.name),
        plan.root.display()
    );
    crate::out!();
    crate::out!("{}", color::dim("next steps:"));
    crate::out!("{}", color::dim(&format!("  cd {}", plan.root.display())));
    crate::out!(
        "{}",
        color::dim(&format!("  scaffold describe {preset_name}"))
    );
}

/// Initialize a git repository, warning instead of failing without git.
fn init_git(root: &Path) {
    match Command::new("git")
        .args(["init", "-q", "-b", "main"])
        .current_dir(root)
        .status()
    {
        Ok(status) if status.success() => {
            crate::out!("{}", color::dim("initialized git repository"))
        }
        Ok(status) => eprintln!(
            "{}",
            color::yellow(&format!("warning: git init exited with {status}"))
        ),
        Err(err) => eprintln!(
            "{}",
            color::yellow(&format!("warning: could not run git: {err}"))
        ),
    }
}
