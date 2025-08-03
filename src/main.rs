mod build;
mod cli;
mod compiler;
mod deploy;
mod install;
mod language;
mod manifest;
mod new;
mod toolchain;
mod utils;

#[tokio::main]
async fn main() {
    if let Err(e) = cli::run().await {
        utils::log_err(e);
        std::process::exit(1);
    }
}
