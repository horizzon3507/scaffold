use clap::Parser;
use scaffold_cli::args::{Cli, Command};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::New(args) => println!("new {}", args.name),
    }
}
