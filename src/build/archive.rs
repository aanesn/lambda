use std::{fs::File, path::PathBuf};

use zip::{ZipWriter, write::SimpleFileOptions};

pub fn zip(binary: &PathBuf, output_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(output_dir)?;

    let bootstrap = &output_dir.join("bootstrap.zip");
    let file = File::create(bootstrap)?;

    let mut zip = ZipWriter::new(file);
    let zopts = SimpleFileOptions::default().unix_permissions(0o755);

    zip.start_file("bootstrap", zopts)?;
    std::io::copy(&mut File::open(binary)?, &mut zip)?;

    zip.finish()?;
    Ok(bootstrap.clone())
}
