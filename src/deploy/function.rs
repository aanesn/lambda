use aws_config::SdkConfig;
use aws_sdk_iam::{error::SdkError, primitives::Blob};
use aws_sdk_lambda::{
    Client as LambdaClient,
    operation::get_function::GetFunctionOutput,
    types::{FunctionCode, Runtime},
};
use std::path::PathBuf;

pub async fn publish(
    cfg: &SdkConfig,
    lambda: &LambdaClient,
    bootstrap: &PathBuf,
    name: &str,
    runtime: &Runtime,
    handler: &str,
    arn: &str,
    arm64: &bool,
) -> anyhow::Result<()> {
    let code = read_archive(bootstrap)?;

    if let Some(curr) = check_function(&lambda, name).await? {}

    Ok(())
}

fn read_archive(bootstrap: &PathBuf) -> anyhow::Result<FunctionCode> {
    let buf = std::fs::read(bootstrap)?;
    let blob = Blob::new(buf);
    let code = FunctionCode::builder().zip_file(blob).build();
    Ok(code)
}

async fn check_function(
    lambda: &LambdaClient,
    name: &str,
) -> anyhow::Result<Option<GetFunctionOutput>> {
    let res = lambda.get_function().function_name(name).send().await;
    match res {
        Ok(curr) => Ok(Some(curr)),
        Err(SdkError::ServiceError(e)) if e.err().is_resource_not_found_exception() => Ok(None),
        Err(_) => anyhow::bail!("failed to fetch lambda"),
    }
}
