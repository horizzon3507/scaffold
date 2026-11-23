//! Command-line parsing.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Scaffolds new projects from opinionated presets.
#[derive(Parser)]
#[command(name = "scaffold", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// What scaffold should do.
#[derive(Subcommand)]
pub enum Command {
    /// Create a new project from a preset.
    New(NewArgs),
    /// List the available presets.
    List(ListArgs),
}

/// Arguments for `scaffold new`.
#[derive(clap::Args)]
pub struct NewArgs {
    /// Project name; also the default directory name.
    pub name: String,

    /// Preset to use. Defaults to the `hory` bundle.
    #[arg(short, long, default_value = "hory")]
    pub preset: String,

    /// Directory to generate into. Defaults to ./<name>.
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Overwrite files that already exist.
    #[arg(short, long)]
    pub force: bool,

    /// Print the plan without writing anything.
    #[arg(long)]
    pub dry_run: bool,

    /// Version control to initialize for the project.
    #[arg(long, value_enum, default_value = "git")]
    pub vcs: Vcs,

    /// Author name recorded in generated metadata.
    #[arg(long)]
    pub author: Option<String>,
}

/// Version control backend for a new project.
#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Vcs {
    /// Run `git init` in the new project.
    Git,
    /// Do not initialize version control.
    None,
}

/// Arguments for `scaffold list`.
#[derive(clap::Args)]
pub struct ListArgs {
    /// Show each preset's description too.
    #[arg(short, long)]
    pub verbose: bool,
}
