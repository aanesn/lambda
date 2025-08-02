use crate::{
    compiler::{self, Compiler},
    language::{self, Language},
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

    println!("building lambda in {} with {}", lang, comp);

    Ok(())
}
