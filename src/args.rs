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
    /// Show the files a preset generates.
    Describe(DescribeArgs),
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

/// Arguments for `scaffold describe`.
#[derive(clap::Args)]
pub struct DescribeArgs {
    /// Preset to describe.
    pub preset: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_new_with_defaults() {
        let cli = Cli::try_parse_from(["scaffold", "new", "demo"]).unwrap();
        let Command::New(args) = cli.command else {
            panic!("expected new")
        };
        assert_eq!(args.name, "demo");
        assert_eq!(args.preset, "hory");
        assert!(!args.force);
        assert!(!args.dry_run);
    }

    #[test]
    fn parses_flags_and_aliases() {
        let cli = Cli::try_parse_from([
            "scaffold", "new", "demo", "-p", "rust", "-d", "/tmp/x", "-f", "--dry-run",
            "--vcs", "none", "--author", "hory",
        ])
        .unwrap();
        let Command::New(args) = cli.command else {
            panic!("expected new")
        };
        assert_eq!(args.preset, "rust");
        assert_eq!(args.dir.unwrap().to_str().unwrap(), "/tmp/x");
        assert!(args.force && args.dry_run);
        assert_eq!(args.author.as_deref(), Some("hory"));
    }

    #[test]
    fn parses_list_and_describe() {
        assert!(Cli::try_parse_from(["scaffold", "list", "-v"]).is_ok());
        assert!(Cli::try_parse_from(["scaffold", "describe", "hory"]).is_ok());
        assert!(Cli::try_parse_from(["scaffold", "nope"]).is_err());
    }
}
