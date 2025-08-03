use crate::{
    language::{self, Language},
    utils,
};
use anyhow::Context;
use aws_config::BehaviorVersion;
use aws_sdk_lambda::{
    primitives::Blob,
    types::{Architecture, FunctionCode, FunctionUrlAuthType, Runtime},
};
use clap::Parser;
use std::path::PathBuf;

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

    #[arg(long, alias = "rt")]
    runtime: Option<Runtime>,

    #[arg(long)]
    iam_role: String,

    #[arg(long)]
    handler: Option<String>,
}

pub async fn deploy(dargs: &DeployArgs) -> anyhow::Result<()> {
    let bootstrap = &dargs.output_dir.join("bootstrap.zip");
    if !bootstrap.exists() {
        anyhow::bail!("failed to find bootstrap, run `lambda build` to build it");
    }

    let lang = match &dargs.language {
        Some(lang) => lang.clone(),
        None => language::detect(&dargs.cwd)?,
    };

    let cfg = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_lambda::Client::new(&cfg);
    let region = cfg.region().unwrap().as_ref();

    let name = match &dargs.name {
        Some(name) => name.clone(),
        None => {
            let manifest_path = &dargs.cwd.join(lang.manifest());
            crate::manifest::get_name(manifest_path)?
        }
    };

    let runtime = match &dargs.runtime {
        Some(rt) => rt.clone(),
        None => lang.runtime(),
    };

    let handler = match &dargs.handler {
        Some(handler) => handler.clone(),
        None => lang.handler().to_string(),
    };

    let code = {
        let buf = std::fs::read(bootstrap)?;
        let blob = Blob::new(buf);
        FunctionCode::builder().zip_file(blob).build()
    };

    let (arch, arch_str) = if dargs.arm64 {
        (Architecture::Arm64, "Arm64")
    } else {
        (Architecture::X8664, "X86")
    };

    let adapter = format!(
        "arn:aws:lambda:{}:753240598075:layer:LambdaAdapterLayer{}:24",
        region, arch_str
    );

    let pb = utils::spinner();
    pb.set_message("creating function...");

    function::create(
        &client,
        &name,
        &runtime,
        &dargs.iam_role,
        &handler,
        &code,
        &adapter,
        &arch,
    )
    .await?;

    pb.finish_and_clear();
    utils::log_timing_sec("created function", &pb.elapsed());

    let pb = utils::spinner();
    pb.set_message("creating function url...");

    let res = &client
        .create_function_url_config()
        .function_name(&name)
        .auth_type(FunctionUrlAuthType::None)
        .send()
        .await
        .with_context(|| anyhow::anyhow!("failed to create function url"))?;

    client
        .add_permission()
        .function_name(&name)
        .statement_id("allow-public-invoke")
        .action("lambda:InvokeFunctionUrl")
        .principal("*")
        .function_url_auth_type(FunctionUrlAuthType::None)
        .send()
        .await
        .with_context(|| anyhow::anyhow!("failed to add public invoke permission"))?;

    pb.finish_and_clear();
    utils::log_timing_sec("created function url", &pb.elapsed());

    utils::log_info("function url", res.function_url());

    Ok(())
}
