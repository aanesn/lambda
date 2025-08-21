use anyhow::Context;
use aws_sdk_iam::error::SdkError;
use aws_sdk_lambda::{Client as LambdaClient, types::FunctionUrlAuthType};
use uuid::Uuid;

pub(crate) async fn upsert(name: &str, lambda: &LambdaClient) -> anyhow::Result<String> {
    if let Some(url) = check_url(name, lambda).await? {
        return Ok(url);
    }

    add_permission(name, lambda).await?;

    let url = create_function_url(name, lambda).await?;
    Ok(url)
}

pub(crate) async fn delete(name: &str, lambda: &LambdaClient) -> anyhow::Result<()> {
    if check_url(name, lambda).await?.is_none() {
        return Ok(());
    }

    lambda
        .delete_function_url_config()
        .function_name(name)
        .send()
        .await
        .context("failed to delete function url")?;

    Ok(())
}

async fn check_url(name: &str, lambda: &LambdaClient) -> anyhow::Result<Option<String>> {
    let res = lambda
        .get_function_url_config()
        .function_name(name)
        .send()
        .await;

    match res {
        Ok(curr) => {
            let url = curr.function_url;
            return Ok(Some(url));
        }
        Err(SdkError::ServiceError(e)) if e.err().is_resource_not_found_exception() => Ok(None),
        Err(_) => anyhow::bail!("failed to fetch function url"),
    }
}

async fn add_permission(name: &str, lambda: &LambdaClient) -> anyhow::Result<()> {
    let statement_id = format!("allow-public-invoke-{}", Uuid::new_v4());
    lambda
        .add_permission()
        .function_name(name)
        .statement_id(statement_id)
        .action("lambda:InvokeFunctionUrl")
        .principal("*")
        .function_url_auth_type(FunctionUrlAuthType::None)
        .send()
        .await
        .context("failed to add public invoke permission to function url")?;

    Ok(())
}

async fn create_function_url(name: &str, lambda: &LambdaClient) -> anyhow::Result<String> {
    let res = lambda
        .create_function_url_config()
        .function_name(name)
        .auth_type(FunctionUrlAuthType::None)
        .send()
        .await
        .context("failed to create function url")?;

    let url = res.function_url;
    Ok(url)
}
