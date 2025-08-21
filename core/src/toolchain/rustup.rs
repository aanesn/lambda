use std::process::{Command, Stdio};

pub(crate) fn check_target(target: &str) -> anyhow::Result<()> {
    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .stdout(Stdio::piped())
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to list installed rust targets: `{}`",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let out = str::from_utf8(&output.stdout)?;
    let exists = out.lines().any(|line| line == target);

    if !exists {
        install_target(target)?;
    }

    Ok(())
}

fn install_target(target: &str) -> anyhow::Result<()> {
    let output = Command::new("rustup")
        .args(["target", "add", target])
        .stdout(Stdio::null())
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to install rust target: `{}`",
            String::from_utf8_lossy(&output.stderr)
        )
    }

    Ok(())
}
