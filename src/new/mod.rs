use crate::{new::framework::Framework, utils};
use clap::Parser;
use std::path::PathBuf;

mod framework;
mod location;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<PathBuf>,

    #[arg(long, alias = "fw")]
    framework: Option<Framework>,
}

pub fn new(nargs: &NewArgs) -> anyhow::Result<()> {
    let rcfg = utils::rcfg();

    let loc = match &nargs.location {
        Some(loc) => loc.clone(),
        None => location::prompt(&rcfg)?,
    };

    let fw = match &nargs.framework {
        Some(fw) => fw.clone(),
        None => framework::prompt(&rcfg)?,
    };

    Ok(())
}
