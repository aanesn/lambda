use crate::language::Language;
use inquire::{Select, ui::RenderConfig};
use std::{fmt::Display, str::FromStr};

#[derive(Clone)]
pub enum Framework {
    Vanilla,
    Axum,
    Actix,
    Rocket,
    Warp,
}

impl Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Framework::Vanilla => write!(f, "vanilla"),
            Framework::Axum => write!(f, "axum"),
            Framework::Actix => write!(f, "actix"),
            Framework::Rocket => write!(f, "rocket"),
            Framework::Warp => write!(f, "warp"),
        }
    }
}

impl FromStr for Framework {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vanilla" => Ok(Framework::Vanilla),
            "axum" => Ok(Framework::Axum),
            "actix" => Ok(Framework::Actix),
            "rocket" => Ok(Framework::Rocket),
            "warp" => Ok(Framework::Warp),
            _ => anyhow::bail!("{} is an unsupported framework", s),
        }
    }
}

impl Framework {
    pub fn from_lang(lang: &Language) -> &[Self] {
        match lang {
            Language::Rust => &[
                Framework::Axum,
                Framework::Actix,
                Framework::Rocket,
                Framework::Warp,
            ],
            Language::Go => &[Framework::Vanilla],
        }
    }
}

pub fn prompt(rcfg: &RenderConfig, lang: &Language) -> anyhow::Result<Framework> {
    let fw = Select::new("framework", Framework::from_lang(lang).to_vec())
        .with_render_config(*rcfg)
        .prompt()?;
    Ok(fw)
}
