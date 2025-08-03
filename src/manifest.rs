use std::path::PathBuf;

pub fn get_name(manifest_path: &PathBuf) -> anyhow::Result<String> {
    let file_name = manifest_path.file_name().unwrap().to_str().unwrap();

    let name = match file_name {
        "Cargo.toml" => cargo_toml(manifest_path)?,
        "go.mod" => go_mod(manifest_path)?,
        _ => anyhow::bail!("`{}` is an unsupported manifest", file_name),
    };

    Ok(name)
}

fn cargo_toml(manifest_path: &PathBuf) -> anyhow::Result<String> {
    let contents = std::fs::read_to_string(&manifest_path)?;
    let parsed: toml::Value = toml::from_str(&contents)?;

    parsed["package"]["name"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("failed to get name from `{}`", manifest_path.display()))
}

fn go_mod(manifest_path: &PathBuf) -> anyhow::Result<String> {
    Ok("todo".to_string())
}
