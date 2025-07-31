use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<PathBuf>,
}

pub fn new(nargs: &NewArgs) -> anyhow::Result<()> {
    println!("creating new project at: {:?}", nargs.location);
    Ok(())
}
