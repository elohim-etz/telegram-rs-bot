use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "get a random number.")]
    Random,
    #[command(description = "echo the input text.")]
    Echo(String),
    #[command(description = "get a random anime quote.")]
    RandomQuote,
    #[command(description = "get a kawaii image. Usage: /kawaii <category>")]
    Kawaii(String),
    #[command(description = "see how sexy you are today")]
    Sexy,
    #[command(description = "see how gay you are today")]
    Gay,
    #[command(description = "get a random piece of advice")]
    Advice,
}

impl Command {
    pub fn help_text() -> String {
        Command::descriptions()
            .to_string()
            .replace("These commands are supported:", "Available commands:")
    }
}