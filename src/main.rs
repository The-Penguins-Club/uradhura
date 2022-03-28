mod fetcher;
mod utils;

use crate::fetcher::fetch_info;
use std::error::Error;
use teloxide::prelude2::*;
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommand;
use crate::utils::{get_sender, validate_url};

type Bot = AutoSend<teloxide::Bot>;

#[derive(BotCommand, Clone)]
#[command(
    rename = "lowercase",
    description = "Fetch post information from Reddit using this bot",
    parse_with = "split"
)]
enum Command {
    #[command(description = "Fetch information from Reddit post and embed it into a message")]
    Rdl { url: String },
}

async fn action(
    bot: Bot,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(reply) = message.reply_to_message() {
        if let Some(url) = reply.text() {
            if let Ok(url) = validate_url(url).await {
                fetch_info(bot, message, url.to_string()).await?;
                return Ok(())
            } else {
                let sender = get_sender(&message);
                bot.send_message(
                    message.chat.id,
                    format!("Invalid url: `{url}`. Sent by: @{sender}"),
                )
                    .parse_mode(ParseMode::Markdown)
                    .send()
                    .await?;
            }
        }
    }

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
