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

fn main() {
    if let Err(e) = cli::run() {
        utils::log_err(e);
        std::process::exit(1);
    }
}
