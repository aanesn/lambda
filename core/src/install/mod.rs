pub(crate) mod zig;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum InstallOpts {
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
    pub(crate) fn cmd(&self) -> &str {
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

    pub(crate) fn zig_args(&self) -> &[&str] {
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

pub(crate) fn opts() -> Vec<InstallOpts> {
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
