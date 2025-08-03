use crate::{
    compiler::{self, Compiler},
    language::{self, Language},
    utils,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct BuildArgs {
    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, alias = "comp")]
    compiler: Option<Compiler>,

    #[arg(long, default_value = ".")]
    cwd: PathBuf,

    #[arg(long)]
    arm64: bool,
}

pub fn build(bargs: &BuildArgs) -> anyhow::Result<()> {
    let lang = match &bargs.language {
        Some(lang) => lang.clone(),
        None => language::detect(&bargs.cwd)?,
    };

    let comp = match &bargs.compiler {
        Some(comp) => comp.clone(),
        None => compiler::detect(&lang),
    };

    let pb = utils::spinner();
    pb.set_message("building...");

    let binary = compiler::exec(&comp, &bargs.cwd, &bargs.arm64)?;

    pb.finish_and_clear();
    utils::log_timing_sec("built", &pb.elapsed());

    utils::log_path("binary", &binary);

    Ok(())
}
