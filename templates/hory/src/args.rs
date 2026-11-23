//! Command-line parsing.

use clap::{Parser, Subcommand};

/// {{Name}}
#[derive(Parser)]
#[command(name = "{{name}}", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// What {{name}} should do.
#[derive(Subcommand)]
pub enum Command {
    /// Run the thing.
    Run,
}
