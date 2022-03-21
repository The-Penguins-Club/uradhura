use std::error::Error;
use html_escape::decode_html_entities;
use teloxide::prelude2::*;
use teloxide::types::{MessageKind, ParseMode, User};
use url::Url;

pub async fn fetch_info(
    bot: crate::Bot,
    msg: Message,
    url: String
) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.delete_message(msg.chat.id, msg.id).send().await?;
    let parsed_url = match Url::parse(&url) {
        Ok(u) => u,
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Invalid url: {}", e.to_string()))
                .send()
                .await?;
            return Ok(());
        }
    };

    if let Some(host) = parsed_url.host() {
        if !host.to_string().contains("reddit.com") {
            bot.send_message(msg.chat.id, format!("Idk about anything other than Reddit"))
                .send()
                .await?;
            return Ok(());
        }
    }

    let query_url = url.clone() + ".json";
    let resp = match reqwest::get(&query_url)
        .await {
        Ok(resp) => resp,
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Couldn't talk to reddit: {}", e.to_string()))
                .send()
                .await?;
            return Ok(());
        }
    };

    let json: serde_json::Value = match resp.json().await {
        Ok(val) => val,
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to understand what reddit said: {}", e.to_string()))
                .send()
                .await?;
            return Ok(());
        }
    };

    let toplevel_data = json.as_array().unwrap().get(0).unwrap().get("data").unwrap();
    let children = toplevel_data.get("children").unwrap();
    let child = children.get(0).unwrap();
    let post = child.get("data").unwrap();

    let title = &decode_html_entities(post.get("title").unwrap().as_str().unwrap());
    let preview_url = match post.get("secure_media") {
        Some(media) if !media.is_null() => {
            let reddit_video = media.get("reddit_video").unwrap();

            reddit_video.get("fallback_url").unwrap().as_str().unwrap()
        },
        _ => post.get("url").unwrap().as_str().unwrap(),
    };
    let subreddit = post.get("subreddit").unwrap().as_str().unwrap();
    let author = post.get("author").unwrap().as_str().unwrap();
    let votes = post.get("score").unwrap().as_i64().unwrap();
    let sender = match msg.kind {
        MessageKind::Common(message) => {
            match message.from {
                None => "deleted_user".to_string(),
                Some(u) => u.username.unwrap(),
            }
        }
        _ => {
            bot.send_message(msg.chat.id, "Unexpected kind of message received")
                .send()
                .await?;

            return Ok(());
        }
    };

    bot.send_message(msg.chat.id, format!("\
    *{title}*[ ]({preview_url})\n\
    By [u/{author}](https://reddit.com/u/{author}) in [r/{subreddit}](subreddit)\n\
    Sent by: @{sender}\n\
    Votes: {votes}\n\
    [Post Link]({url})\
    "))
        .parse_mode(ParseMode::Markdown)
        .send()
        .await?;

    Ok(())
}