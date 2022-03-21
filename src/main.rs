use futures_util::stream::StreamExt;
use telegram_bot::*;
use url::Url;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::new(bot_token);

    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter("telegram_bot=trace")
            .finish(),
    )
        .unwrap();

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                let cmd = data.split_ascii_whitespace().collect::<Vec<&str>>();

                if cmd[0] == "/embed_reddit" {
                    if cmd.len() < 2 {
                        api.send(
                            message.text_reply("You didn't provide reddit link :expressionless:"),
                        )
                        .await?;
                        continue;
                    }

                    match url::Url::parse(cmd[1]) {
                        Ok(u) => {
                            if !u.has_host() {
                                api.send(message.text_reply("Url without host?")).await?;
                                continue;
                            }

                            let host = u.host().unwrap().to_string();
                            if !host.contains("reddit.com") {
                                api.send(
                                    message.text_reply("Idk about anything other than reddit"),
                                )
                                .await?;
                                continue;
                            }

                            let req = reqwest::blocking::get(cmd[1].to_owned() + ".json");

                            if let Ok(res) = req {
                                let val: serde_json::Value = match res.json() {
                                    Ok(v) => v,
                                    Err(_) => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                };

                                let arr = match val.as_array() {
                                    Some(arr) => arr,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                };

                                let inf = match arr.get(0) {
                                    Some(inf) => match inf.as_object() {
                                        None => {
                                            api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                                .await?;
                                            continue;
                                        }
                                        Some(inf) => inf,
                                    },
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                };

                                let data = match inf.get("data") {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                };

                                let children = match data.get("children") {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                };

                                let children = match children.as_array() {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                };

                                let child = match children.get(0) {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                };

                                let post = match child.get("data") {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                };

                                let title = match post.get("title") {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                }.as_str().unwrap();

                                let author = match post.get("author") {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                }.as_str().unwrap();

                                let subreddit = match post.get("subreddit") {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                }.as_str().unwrap();

                                let score = match post.get("score") {
                                    Some(val) => val,
                                    None => {
                                        api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                            .await?;
                                        continue;
                                    }
                                }.as_i64().unwrap();

                                let preview = match post.get("secure_media") {
                                    Some(media) if !media.is_null() => {
                                        let reddit_video = match media.get("reddit_video") {
                                            Some(val) => val,
                                            None => {
                                                api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                                    .await?;
                                                continue;
                                            }
                                        };

                                        match reddit_video.get("fallback_url") {
                                            Some(val) => val,
                                            None => {
                                                api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                                    .await?;
                                                continue;
                                            }
                                        }
                                    }
                                    _ => match post.get("url_overridden_by_dest") {
                                        Some(val) => val,
                                        None => {
                                            api.send(message.text_reply("Failed to ask reddit for info, did you provide valid link?"))
                                                .await?;
                                            continue;
                                        }
                                    }
                                }.as_str().unwrap();

                                let title = html_escape::decode_html_entities(&title);
                                let text = format!(
                                    "\
                                *{title}* [ ]({preview})\n\
                                By [u/{author}](https://reddit.com/u/{author}) in [r/{subreddit}](https://reddit.com/r/{subreddit})\n\
                                Sent by: @{}\n\
                                Votes: {score}\n\
                                [Post Link]({})",
                                    message.from.username.as_ref().unwrap(),
                                    Url::parse(cmd[1]).unwrap().to_string()
                                );

                                api.send(
                                    message
                                        .chat
                                        .text(text)
                                        .parse_mode(ParseMode::Markdown),
                                )
                                .await?;

                                let _ = api.send(message.delete())
                                    .await;
                            } else {
                                api.send(message.text_reply(
                                    "Failed to ask reddit for info, did you provide valid link?",
                                ))
                                .await?;
                            }
                        }
                        Err(e) => {
                            api.send(message.text_reply(format!(
                                "You provided invalid url: `{}`",
                                e.to_string()
                            )))
                            .await?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
