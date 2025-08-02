use clap::Parser;

#[derive(Parser)]
pub struct BuildArgs {}

pub fn build(bargs: &BuildArgs) -> anyhow::Result<()> {
    Ok(())
}
