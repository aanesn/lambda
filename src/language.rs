use inquire::{Select, ui::RenderConfig};
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Clone, PartialEq)]
pub enum Language {
    Rust,
    Go,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::Go => write!(f, "go"),
        }
    }
}

impl FromStr for Language {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Language::Rust),
            "go" | "golang" => Ok(Language::Go),
            _ => anyhow::bail!("`{}` is an unsupported language", s),
        }
    }
}

impl Language {
    pub const ALL: &[Self] = &[Language::Rust, Language::Go];

    pub fn manifest(&self) -> &str {
        match self {
            Language::Rust => "Cargo.toml",
            Language::Go => "go.mod",
        }
    }
}

pub fn prompt(rcfg: &RenderConfig) -> anyhow::Result<Language> {
    let lang = Select::new("language", Language::ALL.to_vec())
        .with_render_config(*rcfg)
        .prompt()?;
    Ok(lang)
}

pub fn detect(cwd: &PathBuf) -> anyhow::Result<Language> {
    if cwd.join("Cargo.toml").exists() {
        return Ok(Language::Rust);
    }
    if cwd.join("go.mod").exists() {
        return Ok(Language::Rust);
    }
    anyhow::bail!("failed to auto-detect language, use --language to override")
}
