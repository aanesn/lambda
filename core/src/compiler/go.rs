use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub(crate) fn build(cwd: &PathBuf, arm64: &bool) -> anyhow::Result<PathBuf> {
    let arch = if *arm64 { "arm64" } else { "amd64" };

    let output = Command::new("go")
        .args(["build", "-o", "bootstrap"])
        .current_dir(cwd)
        .env("GOOS", "linux")
        .env("GOARCH", arch)
        .stdout(Stdio::null())
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to build go: `{}`",
            String::from_utf8_lossy(&output.stderr)
        );
    };

    let binary = cwd.join("bootstrap");
    Ok(binary)
}
