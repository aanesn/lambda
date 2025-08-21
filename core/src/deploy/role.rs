use anyhow::Context;
use aws_config::SdkConfig;
use aws_sdk_iam::{Client as IamClient, error::SdkError};

const ROLE_NAME: &str = "lambda-role";
const LAMBDA_BASIC_EXECUTION: &str =
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole";

pub(crate) async fn upsert(cfg: &SdkConfig) -> anyhow::Result<String> {
    let iam = IamClient::new(cfg);

    if let Some(arm) = check_role(&iam).await? {
        return Ok(arm);
    }

    let arn = create_role(&iam).await?;

    attach_policy(&iam).await?;

    Ok(arn)
}

async fn check_role(iam: &IamClient) -> anyhow::Result<Option<String>> {
    let res = iam.get_role().role_name(ROLE_NAME).send().await;
    match res {
        Ok(curr) => {
            let role = curr
                .role
                .ok_or_else(|| anyhow::anyhow!("missing role in response"))?;
            let arn = role.arn().to_string();
            return Ok(Some(arn));
        }
        Err(SdkError::ServiceError(e)) if e.err().is_no_such_entity_exception() => Ok(None),
        Err(_) => anyhow::bail!("failed to fetch role"),
    }
}

async fn create_role(iam: &IamClient) -> anyhow::Result<String> {
    let policy = serde_json::json!({
        "Version": "2012-10-17",
        "Statement": [
            {
                "Effect": "Allow",
                "Action": "sts:AssumeRole",
                "Principal": {
                    "Service": "lambda.amazonaws.com"
                }
            }
        ]
    });

    let res = iam
        .create_role()
        .role_name(ROLE_NAME)
        .assume_role_policy_document(policy.to_string())
        .send()
        .await
        .context("failed to create role")?;

    let role = res
        .role
        .ok_or_else(|| anyhow::anyhow!("missing role in response"))?;

    let arn = role.arn().to_string();
    return Ok(arn);
}

async fn attach_policy(iam: &IamClient) -> anyhow::Result<()> {
    iam.attach_role_policy()
        .role_name(ROLE_NAME)
        .policy_arn(LAMBDA_BASIC_EXECUTION)
        .send()
        .await
        .context("failed to attach lambda basic execution policy to role")?;

    Ok(())
}
