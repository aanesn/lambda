use crate::{
    framework::{self, Framework},
    language::{self, Language},
    location, template, utils,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct InitArgs {
    location: Option<PathBuf>,

    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, alias = "fw")]
    framework: Option<Framework>,

    #[arg(long)]
    name: Option<String>,
}

pub fn init(iargs: &InitArgs) -> anyhow::Result<()> {
    let rcfg = utils::rcfg();

    let loc = match &iargs.location {
        Some(loc) => loc.clone(),
        None => std::env::current_dir()?,
    };
    location::check_loc(&loc)?;

    let name = match &iargs.name {
        Some(name) => name.clone(),
        None => location::get_name(&loc)?,
    };
    location::check_name(&name)?;

    let lang = match &iargs.language {
        Some(lang) => lang.clone(),
        None => language::prompt(&rcfg)?,
    };

    let fw = match &iargs.framework {
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
