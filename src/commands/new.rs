use crate::{
    framework::{self, Framework},
    language::{self, Language},
    location, template, utils,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<PathBuf>,

    #[arg(long, alias = "lang")]
    language: Option<Language>,

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

    let lang = match &nargs.language {
        Some(lang) => lang.clone(),
        None => language::prompt(&rcfg)?,
    };

    let fw = match &nargs.framework {
        Some(fw) => fw.clone(),
        None => framework::prompt(&rcfg, &lang)?,
    };

    let pb = utils::spinner();
    pb.set_message("scaffolding...");
    template::scaffold(&loc, &name, &lang, &fw)?;
    pb.finish_and_clear();
    utils::log_info(&format!("scaffolded in {:.1}ms", utils::ms(&pb.elapsed())));

    Ok(())
}
