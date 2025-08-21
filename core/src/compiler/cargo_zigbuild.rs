use cargo_zigbuild::Build as ZigBuild;
use std::path::PathBuf;

pub(crate) fn cc(cwd: &PathBuf, arm64: &bool) -> anyhow::Result<PathBuf> {
    crate::install::zig::check()?;
    let manifest_path = cwd.join("Cargo.toml");

    let target = if *arm64 {
        "aarch64-unknown-linux-gnu"
    } else {
        "x86_64-unknown-linux-gnu"
    };
    crate::toolchain::rustup::check_target(&target)?;

    let zig_build = {
        let mut zb = ZigBuild::new(Some(manifest_path.clone()));
        zb.common.target = vec![target.to_string()];
        zb.common.quiet = true;
        zb.release = true;
        zb
    };
    zig_build.execute()?;

    let name = crate::manifest::get_name(&manifest_path)?;
    let binary = cwd.join("target").join(target).join("release").join(name);
    Ok(binary)
}
