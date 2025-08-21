mod cli;

#[tokio::main]
async fn main() {
    if let Err(e) = cli::run().await {
        ld_core::utils::log::err(e);
        std::process::exit(1);
    }
}
