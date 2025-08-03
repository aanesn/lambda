use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn build(cwd: &PathBuf, arm64: &bool) -> anyhow::Result<PathBuf> {
    let goarch = if *arm64 { "arm64" } else { "amd64" };

    let output = Command::new("go")
        .args(["build", "-o", "bootstrap"])
        .current_dir(cwd)
        .env("GOOS", "linux")
        .env("GOARCH", goarch)
        .stdout(Stdio::null())
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to build go: `{}`",
            String::from_utf8_lossy(&output.stderr)
        );
    };

    Ok(cwd.clone())
}
