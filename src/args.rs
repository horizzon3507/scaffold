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
}

/// Arguments for `scaffold new`.
#[derive(clap::Args)]
pub struct NewArgs {
    /// Project name; also the default directory name.
    pub name: String,

    /// Preset to use. Defaults to the `hory` bundle.
    #[arg(long, default_value = "hory")]
    pub preset: String,

    /// Directory to generate into. Defaults to ./<name>.
    #[arg(long)]
    pub dir: Option<PathBuf>,
}
