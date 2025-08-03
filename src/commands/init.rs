use crate::{
    compiler::Compiler,
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

    #[arg(long, alias = "comp")]
    compiler: Option<Compiler>,

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

    let manifest = lang.manifest();
    if loc.join(manifest).exists() {
        anyhow::bail!(
            "`lambda init` cannot be run in a directory with an existing `{}` manifest",
            manifest
        )
    }

    let comp = match &iargs.compiler {
        Some(comp) => comp.clone(),
        None => Compiler::from_lang(&lang),
    };

    let fw = match &iargs.framework {
        Some(fw) => fw.clone(),
        None => framework::prompt(&rcfg, &lang)?,
    };

    let pb = utils::spinner();
    pb.set_message("scaffolding...");

    template::scaffold(&loc, &name, &lang, &fw)?;

    pb.finish_and_clear();
    utils::log_timing_ms("scaffolded", &pb.elapsed());

    utils::log_info(
        "run server",
        &format!("`cd {} && {}`", loc.display(), comp.run_cmd()),
    );

    Ok(())
}
