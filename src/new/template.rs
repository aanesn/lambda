use crate::new::framework::Framework;
use std::path::PathBuf;

#[derive(rust_embed::Embed)]
#[folder = "templates"]
struct Templates;

pub fn scaffold(loc: &PathBuf, name: &str, fw: &Framework) -> anyhow::Result<()> {
    std::fs::create_dir_all(loc)?;

    let template = fw.to_string();

    for path in Templates::iter() {
        if !path.starts_with(&template) {
            continue;
        }

        let file = Templates::get(&path).unwrap();
        let content = std::str::from_utf8(file.data.as_ref())?;
        let processed = content.replace("{% name %}", name);

        let rel = path.strip_prefix(&format!("{}/", template)).unwrap();
        let dest = loc.join(rel);

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if !dest.exists() {
            std::fs::write(dest, processed)?;
        }
    }

    Ok(())
}
