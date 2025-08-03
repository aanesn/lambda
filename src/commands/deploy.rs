use crate::language::Language;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct DeployArgs {
    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, short = 'o', default_value = ".lambda")]
    output_dir: PathBuf,

    #[arg(long)]
    arm64: bool,
}

pub fn deploy(dargs: &DeployArgs) -> anyhow::Result<()> {
    Ok(())
}
