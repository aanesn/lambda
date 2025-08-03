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
        .function_name(name)
        .runtime(runtime.clone())
        .role(iam_role)
        .handler(handler)
        .code(code.clone())
        .layers(adapter)
        .architectures(arch.clone())
        .send()
        .await?;

    Ok(())
}
