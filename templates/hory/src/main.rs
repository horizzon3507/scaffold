//! {{Name}}

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

use crate::args::{Cli, Command};

mod args;

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
        Command::Run => {
            println!("{{name}}");
            Ok(())
        }
    }
}
