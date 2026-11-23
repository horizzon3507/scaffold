use anyhow::Result;
use clap::Parser;
use scaffold_cli::args::{Cli, Command};
use scaffold_cli::preset;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::New(args) => {
            println!("new {}", args.name);
            Ok(())
        }
        Command::List(args) => preset::run_list(&args),
        Command::Describe(args) => preset::run_describe(&args),
    }
}
