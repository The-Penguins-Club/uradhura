mod fetcher;

use std::error::Error;
use teloxide::prelude2::*;
use teloxide::utils::command::BotCommand;
use crate::fetcher::fetch_info;

type Bot = AutoSend<teloxide::Bot>;

#[derive(BotCommand, Clone)]
#[command(
    rename = "lowercase",
    description = "Fetch post information from Reddit using this bot",
    parse_with = "split"
)]
enum Command {
    #[command(description = "Fetch information from Reddit post and embed it into a message")]
    Rdl {
        url: String,
    }
}

async fn action(
    bot: Bot,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Rdl { url } => fetch_info(bot, message, url).await?,
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();

    let bot = teloxide::Bot::from_env().auto_send();

    teloxide::repls2::commands_repl(bot, action, Command::ty()).await;
}
