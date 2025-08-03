use crate::{
    compiler::{self, Compiler},
    language::{self, Language},
    utils,
};
use clap::Parser;
use std::{fs::File, path::PathBuf};
use zip::{ZipWriter, write::SimpleFileOptions};

#[derive(Parser)]
pub struct BuildArgs {
    #[arg(long, alias = "lang")]
    language: Option<Language>,

    #[arg(long, alias = "comp")]
    compiler: Option<Compiler>,

    #[arg(long, default_value = ".")]
    cwd: PathBuf,

    #[arg(long, short = 'o', default_value = ".lambda")]
    output_dir: PathBuf,

    #[arg(long)]
    arm64: bool,
}

pub fn build(bargs: &BuildArgs) -> anyhow::Result<()> {
    let lang = match &bargs.language {
        Some(lang) => lang.clone(),
        None => language::detect(&bargs.cwd)?,
    };

    let comp = match &bargs.compiler {
        Some(comp) => comp.clone(),
        None => compiler::detect(&lang),
    };

    let pb = utils::spinner();
    pb.set_message("building...");

    let binary = compiler::exec(&comp, &bargs.cwd, &bargs.arm64)?;

    pb.finish_and_clear();
    utils::log_timing_sec("built", &pb.elapsed());

    let pb = utils::spinner();
    pb.set_message("zipping...");

    let bootstrap = zip(&binary, &bargs.output_dir)?;

    pb.finish_and_clear();
    utils::log_timing_ms("zipped", &pb.elapsed());

    utils::log_path("bootstrap", &bootstrap);

    Ok(())
}

pub fn zip(binary: &PathBuf, output_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(output_dir)?;

    let bootstrap = &output_dir.join("bootstrap.zip");
    let file = File::create(bootstrap)?;
    let mut zip = ZipWriter::new(file);

    let opts = SimpleFileOptions::default().unix_permissions(0o755);
    zip.start_file("bootstrap", opts)?;
    std::io::copy(&mut File::open(binary)?, &mut zip)?;

    zip.finish()?;
    Ok(bootstrap.clone())
}
