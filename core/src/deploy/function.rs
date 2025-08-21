use crate::language::Language;
use anyhow::Context;
use aws_sdk_iam::{error::SdkError, primitives::Blob};
use aws_sdk_lambda::{
    Client as LambdaClient,
    operation::get_function::GetFunctionOutput,
    types::{Architecture, FunctionCode, LastUpdateStatus, Runtime, State},
};
use clap::Args;
use std::{path::PathBuf, time::Duration};

#[derive(Args)]
pub struct FunctionOpts {
    #[arg(long)]
    runtime: Option<Runtime>,

    #[arg(long)]
    handler: Option<String>,

    #[arg(long, alias = "desc")]
    description: Option<String>,

    #[arg(long)]
    timeout: Option<i32>,

    #[arg(long)]
    memory: Option<i32>,
}

pub(crate) async fn publish(
    bootstrap: &PathBuf,
    lang: &Language,
    name: &str,
    lambda: &LambdaClient,
    arn: &str,
    region: &str,
    fopts: &FunctionOpts,
    arm64: &bool,
) -> anyhow::Result<()> {
    let blob = Blob::new(std::fs::read(bootstrap)?);

    let (arch, arch_str) = if *arm64 {
        (Architecture::Arm64, "Arm64")
    } else {
        (Architecture::X8664, "X86")
    };

    if let Some(curr) = check_function(&lambda, name).await? {
        update_function_config(name, &lambda, arn, fopts, curr).await?;

        wait_for_ready_state(name, &lambda).await?;

        update_function_code(name, &lambda, &blob, &arch).await?;
    } else {
        create_function(
            lang, name, lambda, arn, region, &blob, &arch, arch_str, fopts,
        )
        .await?;
    }

    Ok(())
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

async fn update_function_config(
    name: &str,
    lambda: &LambdaClient,
    arn: &str,
    fopts: &FunctionOpts,
    curr: GetFunctionOutput,
) -> anyhow::Result<()> {
    let config = curr
        .configuration
        .ok_or_else(|| anyhow::anyhow!("missing function config"))?;

    let mut builder = lambda.update_function_configuration().function_name(name);

    if arn != config.role.unwrap_or_default() {
        builder = builder.role(arn);
    }

    if let Some(runtime) = &fopts.runtime {
        if fopts.runtime != config.runtime {
            builder = builder.runtime(runtime.clone());
        }
    }

    if let Some(handler) = &fopts.handler {
        if fopts.handler != config.handler {
            builder = builder.handler(handler.clone());
        }
    }

    if let Some(desc) = &fopts.description {
        if fopts.description != config.description {
            builder = builder.description(desc.clone());
        }
    }

    if let Some(timeout) = &fopts.timeout {
        if fopts.timeout != config.timeout {
            builder = builder.timeout(timeout.clone())
        }
    }

    if let Some(memory) = &fopts.memory {
        if fopts.memory != config.memory_size {
            builder = builder.memory_size(memory.clone())
        }
    }

    builder
        .send()
        .await
        .context("failed to update function config")?;

    Ok(())
}

async fn wait_for_ready_state(name: &str, lambda: &LambdaClient) -> anyhow::Result<()> {
    for attempt in 1..=5 {
        tokio::time::sleep(Duration::from_secs(attempt * 2)).await;

        let res = lambda
            .get_function_configuration()
            .function_name(name)
            .send()
            .await
            .context("failed to fetch function config")?;

        let state = res
            .state
            .ok_or_else(|| anyhow::anyhow!("failed to get function state"))?;

        match (state, res.last_update_status) {
            (State::Pending, _) => {}
            (_, Some(LastUpdateStatus::InProgress)) => {}
            (
                State::Active | State::Failed | State::Inactive,
                Some(LastUpdateStatus::Successful | LastUpdateStatus::Failed) | None,
            ) => break,
            (State::Active | State::Failed | State::Inactive, other) => {
                anyhow::bail!("unexpected function status: `{:?}`", other)
            }
            (other, _) => {
                anyhow::bail!("unexpected function state: `{:?}`", other)
            }
        };

        if attempt == 5 {
            anyhow::bail!(
                "function config didn't update in time, try again in a couple of minutes"
            );
        }
    }

    Ok(())
}

async fn update_function_code(
    name: &str,
    lambda: &LambdaClient,
    blob: &Blob,
    arch: &Architecture,
) -> anyhow::Result<()> {
    lambda
        .update_function_code()
        .function_name(name)
        .zip_file(blob.clone())
        .architectures(arch.clone())
        .send()
        .await
        .context("failed to update function code")?;

    Ok(())
}

async fn create_function(
    lang: &Language,
    name: &str,
    lambda: &LambdaClient,
    arn: &str,
    region: &str,
    blob: &Blob,
    arch: &Architecture,
    arch_str: &str,
    fopts: &FunctionOpts,
) -> anyhow::Result<()> {
    let runtime = match &fopts.runtime {
        Some(runtime) => runtime.clone(),
        None => lang.runtime(),
    };

    let handler = match &fopts.handler {
        Some(handler) => handler.clone(),
        None => lang.handler().to_string(),
    };

    let code = FunctionCode::builder().zip_file(blob.clone()).build();

    let adapter = format!(
        "arn:aws:lambda:{}:753240598075:layer:LambdaAdapterLayer{}:24",
        region, arch_str
    );

    lambda
        .create_function()
        .function_name(name)
        .runtime(runtime)
        .role(arn)
        .handler(handler)
        .code(code)
        .set_description(fopts.description.clone())
        .set_timeout(fopts.timeout.clone())
        .set_memory_size(fopts.memory.clone())
        .layers(adapter)
        .architectures(arch.clone())
        .send()
        .await
        .context("failed to create function")?;

    Ok(())
}
