mod bot;
mod error;
mod utils;

#[tokio::main]
async fn main() {
    if let Err(e) = bot::run_bot().await {
        log::error!("Bot error: {}", e);
        std::process::exit(1);
    }
}