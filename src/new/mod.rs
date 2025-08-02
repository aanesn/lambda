use crate::{new::framework::Framework, utils};
use clap::Parser;
use indicatif::HumanDuration;
use std::path::PathBuf;

mod framework;
mod location;
mod template;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<PathBuf>,

    #[arg(long, alias = "fw")]
    framework: Option<Framework>,

    #[arg(long)]
    name: Option<String>,
}

pub fn new(nargs: &NewArgs) -> anyhow::Result<()> {
    let rcfg = utils::rcfg();

    let loc = match &nargs.location {
        Some(loc) => {
            location::check_loc(loc)?;
            loc.clone()
        }
        None => location::prompt(&rcfg)?, // checks loc internally
    };

    if loc.exists() {
        anyhow::bail!(
            "`{}` already exists, use `lambda init` to initialize the directory",
            loc.display()
        );
    }

    let name = match &nargs.name {
        Some(name) => name.clone(),
        None => location::get_name(&loc)?,
    };
    location::check_name(&name)?;

    let fw = match &nargs.framework {
        Some(fw) => fw.clone(),
        None => framework::prompt(&rcfg)?,
    };

    let pb = utils::spinner();
    pb.set_message("scaffolding...");
    template::scaffold()?;
    pb.finish_and_clear();
    utils::log_info(&format!("scaffolded in {:.1}s", pb.elapsed().as_secs_f32()));

    Ok(())
}
