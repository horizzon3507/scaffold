//! scaffold: project generator with opinionated presets.

use std::process::ExitCode;

use scaffold_cli::cli::{self, Parsed};
use scaffold_cli::color;
use scaffold_cli::completions::{self, Shell};
use scaffold_cli::error::AppResult as Result;
use scaffold_cli::quiet;
use scaffold_cli::{generate, preset};

fn run_completions(args: cli::CompletionsArgs) -> Result<()> {
    let shell = Shell::parse(&args.shell)
        .ok_or_else(|| format!("unknown shell '{}' (try bash, zsh, or fish)", args.shell))?;
    print!("{}", completions::script(shell));
    Ok(())
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    color::init(cli::no_color_requested(&argv));
    quiet::init(cli::quiet_requested(&argv));
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
                eprintln!("{} {err:#}", color::red("error:"));
                ExitCode::FAILURE
            }
        },
        Err(msg) => {
            eprintln!("{} {msg}", color::red("error:"));
            ExitCode::FAILURE
        }
    }
}

fn run(command: cli::Command) -> Result<()> {
    match command {
        cli::Command::New(args) => generate::run_new(&args),
        cli::Command::List(args) => preset::run_list(&args),
        cli::Command::Describe(args) => preset::run_describe(&args),
        cli::Command::Completions(args) => run_completions(args),
    }
}
