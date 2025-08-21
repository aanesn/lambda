use crate::{
    compiler::{self, Compiler},
    language::{self, Language},
    utils,
};
use clap::Parser;
use std::path::PathBuf;

mod archive;

#[derive(Parser)]
pub struct BuildArgs {
    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, alias = "comp")]
    compiler: Option<Compiler>,

    #[arg(long, default_value = ".")]
    cwd: PathBuf,

    #[arg(long, short = 'o', default_value = ".lambda")]
    output_dir: PathBuf,

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

    let pb = utils::progress::spinner();
    pb.set_message("building...");

    let binary = compiler::exec(&comp, &bargs.cwd, &bargs.arm64)?;

    pb.finish_and_clear();
    utils::log::sec("built", &pb.elapsed());

    let bootstrap = archive::zip(&binary, &bargs.output_dir)?;
    utils::log::path("bootstrap:", &bootstrap);

    Ok(())
}
