use crate::language::Language;
use std::{fmt::Display, path::PathBuf, str::FromStr};

mod cargo;
mod cargo_zigbuild;
mod go;

#[derive(Clone)]
pub enum Compiler {
    Cargo,
    CargoZigbuild,
    Go,
}

impl Display for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Compiler::Cargo => write!(f, "cargo"),
            Compiler::CargoZigbuild => write!(f, "cargo-zigbuild"),
            Compiler::Go => write!(f, "go"),
        }
    }
}

impl FromStr for Compiler {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cargo" => Ok(Compiler::Cargo),
            "cargo-zigbuild" => Ok(Compiler::CargoZigbuild),
            "go" => Ok(Compiler::Go),
            _ => anyhow::bail!("`{}` is an unsupported compiler", s),
        }
    }
}

impl Compiler {
    pub fn run_cmd(&self) -> &str {
        match self {
            Compiler::Cargo => "cargo run",
            Compiler::CargoZigbuild => "cargo run",
            Compiler::Go => "go run .",
        }
    }
}

pub fn detect(lang: &Language) -> Compiler {
    match lang {
        Language::Rust => {
            if cfg!(target_os = "linux") && std::env::var("CI").is_ok() {
                Compiler::Cargo
            } else {
                Compiler::CargoZigbuild
            }
        }
        Language::Go => Compiler::Go,
    }
}

pub fn exec(comp: &Compiler, cwd: &PathBuf, arm64: &bool) -> anyhow::Result<PathBuf> {
    let binary = match comp {
        Compiler::Cargo => cargo::build(cwd, arm64)?,
        Compiler::CargoZigbuild => cargo_zigbuild::cc(cwd, arm64)?,
        Compiler::Go => go::build(cwd, arm64)?,
    };
    Ok(binary)
}
