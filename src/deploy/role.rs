use anyhow::Context;
use aws_config::SdkConfig;
use aws_sdk_iam::Client as IamClient;

const ROLE_NAME: &str = "lambda-role";
const LAMBDA_BASIC_EXECUTION: &str =
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole";

pub async fn create(cfg: &SdkConfig) -> anyhow::Result<String> {
    let iam = IamClient::new(cfg);

    // check if role already exists
    if let Ok(exists) = iam.get_role().role_name(ROLE_NAME).send().await {
        let role = exists
            .role
            .ok_or_else(|| anyhow::anyhow!("missing role in response"))?;
        return Ok(role.arn().to_string());
    }

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

    let role = iam
        .create_role()
        .role_name(ROLE_NAME)
        .assume_role_policy_document(policy.to_string())
        .send()
        .await
        .context("failed to create role")?
        .role
        .ok_or_else(|| anyhow::anyhow!("missing role in response"))?;

    iam.attach_role_policy()
        .role_name(ROLE_NAME)
        .policy_arn(LAMBDA_BASIC_EXECUTION)
        .send()
        .await
        .context("failed to attach lambda basic execution policy to role")?;

    Ok(role.arn().to_string())
}
