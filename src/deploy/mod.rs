use crate::{
    language::{self, Language},
    utils,
};
use aws_sdk_lambda::types::Runtime;
use clap::Parser;
use std::path::PathBuf;

mod config;
mod function;

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

    #[arg(long)]
    name: Option<String>,

    #[arg(long)]
    runtime: Option<Runtime>,

    #[arg(long)]
    iam_role: String,

    #[arg(long)]
    handler: Option<String>,

    #[arg(long)]
    region: Option<String>,

    #[arg(long, default_value = "1")]
    retry: u32,
}

pub async fn deploy(dargs: &DeployArgs) -> anyhow::Result<()> {
    let bootstrap = &dargs.output_dir.join("bootstrap.zip");
    if !bootstrap.exists() {
        anyhow::bail!(
            "failed to find bootstrap `{}`, run `lambda build` to create it",
            bootstrap.display()
        );
    }

    let lang = match &dargs.language {
        Some(lang) => lang.clone(),
        None => language::detect(&dargs.cwd)?,
    };

    let name = match &dargs.name {
        Some(name) => name.clone(),
        None => {
            let manifest_path = &dargs.cwd.join(lang.manifest());
            crate::manifest::get_name(manifest_path)?
        }
    };

    let runtime = match &dargs.runtime {
        Some(runtime) => runtime.clone(),
        None => lang.runtime(),
    };

    let handler = match &dargs.handler {
        Some(handler) => handler.clone(),
        None => lang.handler().to_string(),
    };

    let cfg = config::load(&dargs.region, &dargs.retry).await?;

    let pb = utils::spinner();
    pb.set_message("publishing...");

    function::publish(
        &cfg,
        &bootstrap,
        &dargs.arm64,
        &name,
        &runtime,
        &dargs.iam_role,
        &handler,
    )
    .await?;

    pb.finish_and_clear();
    utils::log_info("published", &utils::sec(&pb.elapsed()));

    Ok(())
}
