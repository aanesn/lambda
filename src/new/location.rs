use inquire::{Text, ui::RenderConfig};
use std::path::PathBuf;

const DEFAULT_LOCATION: &str = "./";

pub fn prompt(rcfg: &RenderConfig) -> anyhow::Result<PathBuf> {
    let text = Text::new("location")
        .with_default(DEFAULT_LOCATION)
        .with_render_config(*rcfg)
        .prompt()?;

    let loc = PathBuf::from(&text);
    Ok(loc)
}
