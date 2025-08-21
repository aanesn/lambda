use cargo_zigbuild::Zig;
use std::process::{Command, Stdio};

pub(crate) fn check() -> anyhow::Result<()> {
    if Zig::find_zig().is_ok() {
        return Ok(());
    }

    #[allow(unused_mut)]
    let mut iopts = crate::install::opts();

    #[cfg(windows)]
    {
        use crate::install::InstallOpts;
        iopts.retain(|iopt| *iopt != InstallOpts::Npm);
    }

    if iopts.is_empty() {
        anyhow::bail!("failed to find install option for zig")
    }

    let iopt = iopts[0];
    let output = Command::new(iopt.cmd())
        .args(iopt.zig_args())
        .stdout(Stdio::null())
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to install zig: `{}`",
            String::from_utf8_lossy(&output.stderr)
        )
    }

    Ok(())
}
