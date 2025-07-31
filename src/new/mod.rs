use crate::utils;
use clap::Parser;
use std::path::PathBuf;

mod location;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<PathBuf>,
}

pub fn new(nargs: &NewArgs) -> anyhow::Result<()> {
    let rcfg = utils::rcfg();

    let loc = match &nargs.location {
        Some(loc) => loc.clone(),
        None => location::prompt(&rcfg)?,
    };

    Ok(())
}
