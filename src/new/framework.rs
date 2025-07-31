use inquire::{Select, ui::RenderConfig};
use std::{fmt::Display, str::FromStr};

#[derive(Clone)]
pub enum Framework {
    Axum,
    Actix,
    Rocket,
    Warp,
    Flask,
    Django,
    FastApi,
    Express,
    Fastify,
}

impl Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Framework::Axum => write!(f, "axum"),
            Framework::Actix => write!(f, "actix"),
            Framework::Rocket => write!(f, "rocket"),
            Framework::Warp => write!(f, "warp"),
            Framework::Flask => write!(f, "flask"),
            Framework::Django => write!(f, "django"),
            Framework::FastApi => write!(f, "fastapi"),
            Framework::Express => write!(f, "express"),
            Framework::Fastify => write!(f, "fastify"),
        }
    }
}

impl FromStr for Framework {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "axum" => Ok(Framework::Axum),
            "actix" => Ok(Framework::Actix),
            "rocket" => Ok(Framework::Rocket),
            "warp" => Ok(Framework::Warp),
            "flask" => Ok(Framework::Flask),
            "django" => Ok(Framework::Django),
            "fastapi" => Ok(Framework::FastApi),
            "express" => Ok(Framework::Express),
            "fastify" => Ok(Framework::Fastify),
            _ => anyhow::bail!("{} is an unsupported framework", s),
        }
    }
}

impl Framework {
    pub const ALL: &[Self] = &[
        Framework::Axum,
        Framework::Actix,
        Framework::Rocket,
        Framework::Warp,
        Framework::Flask,
        Framework::Django,
        Framework::FastApi,
        Framework::Express,
        Framework::Fastify,
    ];
}

pub fn prompt(rcfg: &RenderConfig) -> anyhow::Result<Framework> {
    let fw = Select::new("framework", Framework::ALL.to_vec())
        .with_render_config(*rcfg)
        .prompt()?;

    Ok(fw)
}
