use aws_sdk_lambda::types::{Architecture, FunctionCode, Runtime};

pub async fn create(
    client: &aws_sdk_lambda::Client,
    name: &str,
    runtime: &Runtime,
    iam_role: &str,
    handler: &str,
    code: &FunctionCode,
    adapter: &str,
    arch: &Architecture,
) -> anyhow::Result<()> {
    client
        .create_function()
        .function_name(name.clone())
        .runtime(runtime.clone())
        .role(iam_role.clone())
        .handler(handler)
        .code(code.clone())
        .layers(adapter.clone())
        .architectures(arch.clone())
        .send()
        .await?;

    Ok(())
}
