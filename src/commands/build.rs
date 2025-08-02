use crate::language::{self, Language};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct BuildArgs {
    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, default_value = ".")]
    cwd: PathBuf,
}

pub fn build(bargs: &BuildArgs) -> anyhow::Result<()> {
    let lang = match &bargs.language {
        Some(lang) => lang.clone(),
        None => language::detect(&bargs.cwd)?,
    };

    println!("building lambda in {}", lang);

    Ok(())
}
