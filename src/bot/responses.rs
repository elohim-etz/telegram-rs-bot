pub fn welcome_message() -> String {
    r#"
Welcome to the Rust Telegram Bot! 🦀

I can do several things:
- Respond to commands like /help, /start, /random
- React to certain words like "hello", "thanks", "bye"

Try sending me a message or use /help to see all commands.
    "#.trim().to_string()
}

pub fn help_response() -> String {
    r#"
Need help? Here's what I can do:

- /start - Start interacting with the bot
- /help - Show this help message
- /random - Get a random number
- /echo <text> - Echo back the text

I also respond to keywords like "hello", "thanks", and "bye".
    "#.trim().to_string()
}