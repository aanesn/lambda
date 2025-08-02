mod cli;
mod commands;
mod compiler;
mod framework;
mod language;
mod location;
mod template;
mod utils;

fn main() {
    if let Err(e) = cli::run() {
        utils::log_err(e);
        std::process::exit(1);
    }
}
