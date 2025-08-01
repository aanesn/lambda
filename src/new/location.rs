use inquire::{Text, ui::RenderConfig, validator::Validation};
use std::path::PathBuf;

const DEFAULT_LOCATION: &str = "./lambda";

pub fn prompt(rcfg: &RenderConfig) -> anyhow::Result<PathBuf> {
    let text = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(*rcfg)
        .with_validator(|text: &str| {
            let loc = PathBuf::from(&text);
            match check_loc(&loc) {
                Ok(_) => Ok(Validation::Valid),
                Err(e) => Ok(Validation::Invalid(e.into())),
            }
        })
        .prompt()?;

    let loc = PathBuf::from(&text);
    Ok(loc)
}

pub fn check_loc(loc: &PathBuf) -> anyhow::Result<()> {
    if std::env::join_paths(loc).is_err() {
        anyhow::bail!(
            "location `{}` contains invalid path characters (usually `:` or `\"`)",
            loc.display()
        );
    }

    if loc.is_file() {
        anyhow::bail!("`{}` is a file that already exists", loc.display());
    }

    Ok(())
}
