use crate::{language::Language, new::framework::Framework};
use std::path::PathBuf;

#[derive(rust_embed::Embed)]
#[folder = "templates"]
struct Templates;

pub fn scaffold(loc: &PathBuf, name: &str, lang: &Language, fw: &Framework) -> anyhow::Result<()> {
    std::fs::create_dir_all(loc)?;

    let prefix = format!("{}/{}/", lang, fw);

    for file in Templates::iter().filter(|f| f.starts_with(&prefix)) {
        let path = {
            let rel = file.strip_prefix(&prefix).unwrap();
            loc.join(rel)
        };

        // don't overwrite things
        if path.exists() {
            continue;
        }

        let file = Templates::get(&file).unwrap();
        let content = std::str::from_utf8(file.data.as_ref())?;
        let processed = content.replace("{% name %}", name);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(path, processed)?;
    }

    Ok(())
}
