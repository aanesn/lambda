use crate::{
    deploy::function::FunctionOpts,
    language::{self, Language},
    utils,
};
use aws_sdk_lambda::client::Client as LambdaClient;
use clap::Parser;
use std::path::PathBuf;

mod config;
mod function;
mod role;

#[derive(Parser)]
pub struct DeployArgs {
    #[arg(long, short = 'o', default_value = ".lambda")]
    output_dir: PathBuf,

    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, default_value = ".")]
    cwd: PathBuf,

    #[arg(long)]
    name: Option<String>,

    #[arg(long)]
    region: Option<String>,

    #[arg(long, default_value = "1")]
    retry: u32,

    #[arg(long)]
    role: Option<String>,

    #[command(flatten)]
    fopts: FunctionOpts,

    #[arg(long)]
    arm64: bool,
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

    let cfg = config::load(&dargs.region, &dargs.retry).await?;
    let lambda = LambdaClient::new(&cfg);
    let arn = role::upsert(&cfg).await?;

    let pb = utils::spinner();
    pb.set_message("publishing...");

    function::publish(
        &cfg,
        &lambda,
        &bootstrap,
        &lang,
        &name,
        &arn,
        &dargs.fopts,
        &dargs.arm64,
    )
    .await?;

    pb.finish_and_clear();
    utils::log_info("published", &utils::sec(&pb.elapsed()));

    Ok(())
}
