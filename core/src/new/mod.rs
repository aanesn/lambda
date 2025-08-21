use crate::{
    compiler::{self, Compiler},
    language::{self, Language},
    new::framework::Framework,
    utils,
};
use clap::Parser;
use std::path::PathBuf;

mod config;
mod framework;
mod location;
mod template;

#[derive(Parser)]
pub struct NewArgs {
    location: Option<PathBuf>,

    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, alias = "comp")]
    compiler: Option<Compiler>,

    #[arg(long, alias = "fw")]
    framework: Option<Framework>,

    #[arg(long)]
    name: Option<String>,
}

pub fn new(nargs: &NewArgs) -> anyhow::Result<()> {
    let rcfg = crate::new::config::rcfg();

    let loc = match &nargs.location {
        Some(loc) => {
            location::check_loc(loc)?;
            loc.clone()
        }
        None => location::prompt(&rcfg)?, // checks loc internally
    };
    let abs = std::path::absolute(&loc)?;

    let name = match &nargs.name {
        Some(name) => name.clone(),
        None => location::get_name(&loc, &abs)?,
    };
    location::check_name(&name)?;

    let lang = match &nargs.language {
        Some(lang) => lang.clone(),
        None => language::prompt(&rcfg)?,
    };

    let manifest = lang.manifest();
    if loc.join(manifest).exists() {
        anyhow::bail!(
            "cannot run `lambda new` in a directory with an existing `{}` manifest",
            manifest
        )
    }

    let comp = match &nargs.compiler {
        Some(comp) => comp.clone(),
        None => compiler::detect(&lang),
    };

    let fw = match &nargs.framework {
        Some(fw) => fw.clone(),
        None => framework::prompt(&rcfg, &lang)?,
    };

    let pb = utils::progress::spinner();
    pb.set_message("scaffolding...");

    template::scaffold(&loc, &name, &lang, &fw)?;

    pb.finish_and_clear();
    utils::log::ms("scaffolded", &pb.elapsed());

    if abs == std::env::current_dir()? {
        utils::log::info("run server:", comp.run_cmd());
    } else {
        utils::log::info(
            "run server:",
            &format!("`cd {} && {}`", loc.display(), comp.run_cmd()),
        );
    }

    Ok(())
}
