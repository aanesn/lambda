mod cli;
mod commands;
mod compiler;
mod framework;
mod install;
mod language;
mod location;
mod manifest;
mod template;
mod toolchain;
mod utils;

#[tokio::main]
async fn main() {
    if let Err(e) = cli::run().await {
        utils::log_err(e);
        std::process::exit(1);
    }
}
