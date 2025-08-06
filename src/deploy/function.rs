use aws_config::SdkConfig;
use aws_sdk_lambda::{
    primitives::Blob,
    types::{Architecture, FunctionCode, Runtime},
};
use std::path::PathBuf;

pub async fn publish(
    cfg: &SdkConfig,
    bootstrap: &PathBuf,
    arm64: &bool,
    name: &str,
    runtime: &Runtime,
    iam_role: &str,
    handler: &str,
) -> anyhow::Result<()> {
    let client = aws_sdk_lambda::Client::new(cfg);

    let code = {
        let buf = std::fs::read(bootstrap)?;
        let blob = Blob::new(buf);
        FunctionCode::builder().zip_file(blob).build()
    };

    let (arch, arch_str) = if *arm64 {
        (Architecture::Arm64, "Arm64")
    } else {
        (Architecture::X8664, "X86")
    };

    let adapter = format!(
        "arn:aws:lambda:{}:753240598075:layer:LambdaAdapterLayer{}:24",
        cfg.region().unwrap().as_ref(),
        arch_str
    );

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
