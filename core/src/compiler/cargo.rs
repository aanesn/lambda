use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub(crate) fn build(cwd: &PathBuf, arm64: &bool) -> anyhow::Result<PathBuf> {
    let manifest_path = cwd.join("Cargo.toml");

    let target = if *arm64 {
        "aarch64-unknown-linux-gnu"
    } else {
        "x86_64-unknown-linux-gnu"
    };
    crate::toolchain::rustup::check_target(target)?;

    let output = Command::new("cargo")
        .args(["build", "--release", "--target", target, "--manifest-path"])
        .arg(manifest_path.as_os_str())
        .stdout(Stdio::null())
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to build rust using cargo: `{}`",
            String::from_utf8_lossy(&output.stderr)
        );
    };

    let name = crate::manifest::get_name(&manifest_path)?;
    let binary = cwd.join("target").join(target).join("release").join(name);
    Ok(binary)
}
