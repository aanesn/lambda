use inquire::{Text, ui::RenderConfig, validator::Validation};
use std::path::PathBuf;
use unicode_xid::UnicodeXID;

const DEFAULT_LOCATION: &str = "lambda";
const MAX_NAME_LEN: usize = 214;

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

pub fn get_name(loc: &PathBuf) -> anyhow::Result<String> {
    let name = loc.file_name().ok_or_else(|| {
        anyhow::anyhow!(
            "failed to auto-detect name from location `{}`, use --name to override",
            loc.display()
        )
    })?;

    name.to_str()
        .ok_or_else(|| anyhow::anyhow!("name `{:?}` cannot contain invalid unicode", name))
        .map(|s| s.to_string())
}

pub fn check_name(name: &str) -> anyhow::Result<()> {
    if name.len() > MAX_NAME_LEN {
        anyhow::bail!("name cannot be longer than {} characters", MAX_NAME_LEN);
    }

    let mut chars = name.chars();
    if let Some(c) = chars.next() {
        if !(UnicodeXID::is_xid_start(c)) {
            anyhow::bail!("name `{}` must start with a letter or underscore", name);
        }
    }
    for c in chars {
        if !(UnicodeXID::is_xid_continue(c) || c == '-') {
            anyhow::bail!(
                "name `{}` must only include letters, numbers, underscores and dashes",
                name
            );
        }
    }

    if name.ends_with('-') {
        anyhow::bail!("name `{}` cannot end with a dash", name);
    }

    Ok(())
}
