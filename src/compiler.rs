use crate::language::Language;
use std::{fmt::Display, str::FromStr};

#[derive(Clone)]
pub enum Compiler {
    Cargo,
    Go,
}

impl Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Compiler::Cargo => write!(f, "cargo"),
            Compiler::Go => write!(f, "go"),
        }
    }
}

impl FromStr for Compiler {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cargo" => Ok(Compiler::Cargo),
            "go" => Ok(Compiler::Go),
            _ => anyhow::bail!("`{}` is an unsupported compiler", s),
        }
    }
}

impl Compiler {
    pub fn from_lang(lang: &Language) -> Self {
        match lang {
            Language::Rust => Compiler::Cargo,
            Language::Go => Compiler::Go,
        }
    }

    pub fn run_cmd(&self) -> &str {
        match self {
            Compiler::Cargo => "cargo run",
            Compiler::Go => "go run .",
        }
    }
}
