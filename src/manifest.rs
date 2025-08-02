use crate::language::Language;
use std::path::PathBuf;

pub fn get_name(lang: &Language, manifest_path: &PathBuf) -> anyhow::Result<String> {
    let name = match lang {
        Language::Rust => cargo_toml(manifest_path)?,
        Language::Go => go_mod(manifest_path)?,
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
