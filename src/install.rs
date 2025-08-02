use std::process::Command;

#[derive(Clone, Copy, PartialEq)]
enum InstallOpts {
    #[cfg(not(windows))]
    Homebrew,
    #[cfg(windows)]
    Winget,
    #[cfg(windows)]
    Scoop,
    #[cfg(windows)]
    Choco,
    Npm,
    Pip,
}

impl InstallOpts {
    pub fn cmd(&self) -> &str {
        match self {
            #[cfg(not(windows))]
            InstallOpts::Homebrew => "brew",
            #[cfg(windows)]
            InstallOpts::Winget => "winget",
            #[cfg(windows)]
            InstallOpts::Scoop => "scoop",
            #[cfg(windows)]
            InstallOpts::Choco => "choco",
            InstallOpts::Npm => "npm",
            InstallOpts::Pip => "pip3",
        }
    }

    pub fn zig_args(&self) -> &[&str] {
        match self {
            #[cfg(not(windows))]
            InstallOpts::Homebrew => &["install", "zig"],
            #[cfg(windows)]
            InstallOpts::Winget => &["install", "zig.zig"],
            #[cfg(windows)]
            InstallOpts::Scoop => &["install", "zig"],
            #[cfg(windows)]
            InstallOpts::Choco => &["install", "zig"],
            InstallOpts::Npm => &["install", "-g", "@ziglang/cli"],
            InstallOpts::Pip => &["install", "ziglang"],
        }
    }
}

pub fn zig() -> anyhow::Result<()> {
    #[allow(unused_mut)]
    let mut iopts = install_opts();

    #[cfg(windows)]
    exclude_iopts(&mut iopts, &[InstallOpts::Npm]);

    if iopts.is_empty() {
        anyhow::bail!("failed to find install option for zig")
    }

    let iopt = iopts[0];
    let output = Command::new(iopt.cmd()).args(iopt.zig_args()).output()?;

    if !output.status.success() {
        anyhow::bail!(
            "failed to install zig: `{}`",
            String::from_utf8_lossy(&output.stderr)
        )
    }

    Ok(())
}

fn install_opts() -> Vec<InstallOpts> {
    let mut opts = Vec::new();

    #[cfg(not(windows))]
    if which::which("brew").is_ok() {
        opts.push(InstallOpts::Homebrew);
    }

    #[cfg(windows)]
    if which::which("winget").is_ok() {
        opts.push(InstallOpts::Winget);
    }

    #[cfg(windows)]
    if which::which("scoop").is_ok() {
        opts.push(InstallOpts::Scoop);
    }

    #[cfg(windows)]
    if which::which("choco").is_ok() {
        opts.push(InstallOpts::Choco);
    }

    if which::which("npm").is_ok() {
        opts.push(InstallOpts::Npm);
    }

    if which::which("pip3").is_ok() {
        opts.push(InstallOpts::Pip);
    }

    opts
}

fn exclude_iopts(iopts: &mut Vec<InstallOpts>, rm: &[InstallOpts]) {
    iopts.retain(|iopt| !rm.contains(iopt));
}
