use cargo_zigbuild::{Build as ZigBuild, Zig};
use std::path::PathBuf;

pub fn build(cwd: &PathBuf, arm64: &bool) -> anyhow::Result<PathBuf> {
    let manifest_path = cwd.join("Cargo.toml");

    let target = if *arm64 {
        "aarch64-unknown-linux-gnu"
    } else {
        "x86_64-unknown-linux-gnu"
    };
    crate::toolchain::check_rust_target(&target)?;

    if !Zig::find_zig().is_ok() {
        crate::install::zig()?;
    }

    let zig_build = {
        let mut zb = ZigBuild::new(Some(manifest_path.clone()));
        zb.common.target = vec![target.to_string()];
        zb.common.quiet = true;
        zb.release = true;
        zb
    };
    zig_build.execute()?;

    let dest = cwd.join("target").join(target).join("release");
    Ok(dest)
}
