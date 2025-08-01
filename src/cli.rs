use crate::commands::{
    init::{InitArgs, init},
    new::{NewArgs, new},
};
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
    #[command(about = "scaffold a new lambda in the current directory")]
    Init(InitArgs),
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::New(nargs) => new(nargs)?,
        Command::Init(iargs) => init(iargs)?,
    }

    Ok(())
}
