use inquire::{Select, ui::RenderConfig};
use std::{fmt::Display, str::FromStr};

#[derive(Clone)]
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
