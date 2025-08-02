use std::{path::PathBuf, process::Command};

pub fn build(cwd: &PathBuf, arm64: &bool) -> anyhow::Result<PathBuf> {
    let manifest_path = cwd.join("Cargo.toml");

    let target = if *arm64 {
        "aarch64-unknown-linux-gnu"
    } else {
        "x86_64-unknown-linux-gnu"
    };
    crate::toolchain::check_rust_target(target)?;

    let output = Command::new("cargo")
        .args(["build", "--release", "--target", target, "--manifest-path"])
        .arg(manifest_path.as_os_str())
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to build rust using cargo: `{}`",
            String::from_utf8_lossy(&output.stderr)
        );
    };

    let dest = cwd.join("target").join(target).join("release");
    Ok(dest)
}
