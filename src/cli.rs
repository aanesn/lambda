use crate::commands::new::{NewArgs, new};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "scaffold a new lambda")]
    New(NewArgs),
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::New(nargs) => new(nargs)?,
    }

    Ok(())
}
