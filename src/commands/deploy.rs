use crate::language::{self, Language};
use aws_config::BehaviorVersion;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct DeployArgs {
    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, default_value = ".")]
    cwd: PathBuf,

    #[arg(long, short = 'o', default_value = ".lambda")]
    output_dir: PathBuf,

    #[arg(long)]
    arm64: bool,
}

pub async fn deploy(dargs: &DeployArgs) -> anyhow::Result<()> {
    let lang = match &dargs.language {
        Some(lang) => lang.clone(),
        None => language::detect(&dargs.cwd)?,
    };

    let cfg = aws_config::load_defaults(BehaviorVersion::latest()).await;
    println!("region: {:?}", cfg.region());

    Ok(())
}
