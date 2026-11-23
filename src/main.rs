//! scaffold: project generator with opinionated presets.

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use scaffold_cli::args::{Cli, Command};
use scaffold_cli::{generate, preset};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::New(args) => generate::run_new(&args),
        Command::List(args) => preset::run_list(&args),
        Command::Describe(args) => preset::run_describe(&args),
    }
}
