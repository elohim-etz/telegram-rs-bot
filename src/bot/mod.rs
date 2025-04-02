// Declare submodules
pub mod commands;
pub mod handlers;
pub mod models;
pub mod responses;
pub mod quotes;
pub mod kawaii; 
pub mod percent; 
pub mod advice;
pub mod waifu;

pub use commands::Command;
pub use handlers::{handle_commands, handle_messages};
pub use models::BotState;

use teloxide::{prelude::*, types::Update};
use teloxide::dptree;
use crate::error::BotError;

pub async fn run_bot() -> Result<(), BotError> {
    let config = crate::utils::config::Config::new()?;
    crate::utils::logging::setup_logging(&config.log_level);

    log::info!("Starting bot...");

    let bot = Bot::new(&config.telegram_bot_token);
    
    let state = BotState { user_count: 0 };

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_commands),
        )
        .branch(dptree::endpoint(handle_messages));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}