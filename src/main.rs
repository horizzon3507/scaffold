//! scaffold: project generator with opinionated presets.

use std::process::ExitCode;

use scaffold_cli::error::AppResult as Result;
use scaffold_cli::cli::{self, Parsed};
use scaffold_cli::{generate, preset};

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    match cli::parse(&argv) {
        Ok(Parsed::Help(text)) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Ok(Parsed::Version) => {
            println!("scaffold {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Ok(Parsed::Run(command)) => match run(command) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("error: {err:#}");
                ExitCode::FAILURE
            }
        },
        Err(msg) => {
            eprintln!("error: {msg}");
            ExitCode::FAILURE
        }
    }
}

fn run(command: cli::Command) -> Result<()> {
    match command {
        cli::Command::New(args) => generate::run_new(&args),
        cli::Command::List(args) => preset::run_list(&args),
        cli::Command::Describe(args) => preset::run_describe(&args),
    }
}
