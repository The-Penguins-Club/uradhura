use html_escape::decode_html_entities;
use image::ImageFormat;
use std::env::temp_dir;
use std::error::Error;
use std::process::Command;
use teloxide::prelude2::*;
use teloxide::types::{InputFile, MessageKind, ParseMode};
use url::Url;

pub async fn fetch_info(
    bot: crate::Bot,
    msg: Message,
    mut url: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.delete_message(msg.chat.id, msg.id).send().await?;
    let sender = match msg.kind {
        MessageKind::Common(message) => match message.from {
            None => "deleted_user".to_string(),
            Some(u) => match u.username {
                Some(username) => username,
                None => {
                    u.first_name
                        + &if let Some(last_name) = &u.last_name {
                            " ".to_string() + last_name
                        } else {
                            "".to_string()
                        }
                }
            },
        },
        _ => {
            bot.send_message(msg.chat.id, "Unexpected kind of message received")
                .send()
                .await?;

            return Ok(());
        }
    };
    let mut parsed_url = match Url::parse(&url) {
        Ok(u) => u,
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Invalid url: `{url}`. Sent by: @{sender}\nError message: {}",
                    e.to_string()
                ),
            )
            .parse_mode(ParseMode::Markdown)
            .send()
            .await?;
            return Ok(());
        }
    };
    parsed_url.set_query(None);
    url = parsed_url.to_string();

    if let Some(host) = parsed_url.host() {
        if !host.to_string().contains("reddit.com") {
            bot.send_message(
                msg.chat.id,
                format!("Invalid url: `{url}`. Sent by: @{sender}"),
            )
            .parse_mode(ParseMode::Markdown)
            .send()
            .await?;
            return Ok(());
        }
    }

    let query_url = url.clone() + ".json";
    let resp = match reqwest::get(&query_url).await {
        Ok(resp) => resp,
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Couldn't talk to reddit: {}.\nUrl: `{url}`.\nSent by: @{sender}",
                    e.to_string()
                ),
            )
            .parse_mode(ParseMode::Markdown)
            .send()
            .await?;
            return Ok(());
        }
    };

    let json: serde_json::Value = match resp.json().await {
        Ok(val) => val,
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "Failed to understand what reddit said: {}.\nUrl: `{url}`.\nSent by: @{sender}",
                    e.to_string()
                ),
            )
            .parse_mode(ParseMode::Markdown)
            .send()
            .await?;
            return Ok(());
        }
    };

    let post = &json[0]["data"]["children"][0]["data"];

    let title = &decode_html_entities(post.get("title").unwrap().as_str().unwrap());
    let subreddit = post.get("subreddit").unwrap().as_str().unwrap();
    let author = post.get("author").unwrap().as_str().unwrap();
    let votes = post.get("score").unwrap().as_i64().unwrap();

    bot.send_message(msg.chat.id, format!("\
    *{title}*\n\
    By [u/{author}](https://reddit.com/u/{author}) in [r/{subreddit}](https://reddit.com/r/{subreddit})\n\
    Sent by: @{sender}\n\
    Votes: {votes}\n\
    [Post Link]({url})\
    "))
        .parse_mode(ParseMode::Markdown)
        .disable_web_page_preview(true)
        .send()
        .await?;

    let sending_message = bot
        .send_message(msg.chat.id, "Getting preview...")
        .send()
        .await?;

    let preview_url = match post.get("secure_media") {
        Some(media) if !media.is_null() => {
            return match media.get("reddit_video") {
                Some(v) => {
                    let hls_url = match v.get("hls_url") {
                        Some(url) => url.as_str().unwrap(),
                        None => {
                            bot.delete_message(sending_message.chat.id, sending_message.id)
                                .send()
                                .await?;
                            bot.send_message(msg.chat.id, "Could not get preview :(")
                                .send()
                                .await?;

                            return Ok(());
                        }
                    };

                    let file_path = format!(
                        "{}/{}.mp4",
                        temp_dir().to_str().unwrap(),
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    );

                    let cmd = Command::new("ffmpeg")
                        .arg("-i")
                        .arg(hls_url)
                        .arg("-c")
                        .arg("copy")
                        .arg(&file_path)
                        .output();

                    match cmd {
                        Ok(o) => {
                            if !o.status.success() {
                                println!("{}", String::from_utf8(o.stderr).unwrap());
                                bot.delete_message(sending_message.chat.id, sending_message.id)
                                    .send()
                                    .await?;
                                bot.send_message(msg.chat.id, "Could not get preview :(")
                                    .send()
                                    .await?;

                                return Ok(());
                            }
                        }
                        Err(_) => {
                            bot.delete_message(sending_message.chat.id, sending_message.id)
                                .send()
                                .await?;
                            bot.send_message(msg.chat.id, "Could not get preview :(")
                                .send()
                                .await?;

                            return Ok(());
                        }
                    }

                    let file = std::fs::read(&file_path).unwrap();
                    let _ = std::fs::remove_file(&file_path);
                    bot.send_video(sending_message.chat.id, InputFile::memory(file))
                        .send()
                        .await?;
                    bot.delete_message(sending_message.chat.id, sending_message.id)
                        .send()
                        .await?;
                    Ok(())
                }
                None => {
                    bot.delete_message(sending_message.chat.id, sending_message.id)
                        .send()
                        .await?;
                    bot.send_message(msg.chat.id, "Could not get preview :(")
                        .send()
                        .await?;

                    Ok(())
                }
            };
        }
        _ => {
            if let Some(u) = post.get("url_overridden_by_dest") {
                u.as_str()
            } else {
                None
            }
        }
    };

    if let None = preview_url {
        bot.send_message(msg.chat.id, "Could not get preview :(")
            .send()
            .await?;

        bot.delete_message(sending_message.chat.id, sending_message.id)
            .send()
            .await?;

        return Ok(());
    }

    let req = match reqwest::get(preview_url.unwrap()).await {
        Ok(resp) => resp,
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("Couldn't get preview from reddit: {}", e.to_string()),
            )
            .send()
            .await?;
            return Ok(());
        }
    };

    let content_type = req
        .headers()
        .get("Content-Type")
        .unwrap()
        .to_str()?
        .to_string();

    let bytes = req.bytes().await.unwrap();

    if content_type == "image/gif" {
        bot.send_animation(
            msg.chat.id,
            InputFile::memory(bytes).file_name("preview.gif"),
        )
        .send()
        .await?;
    } else if content_type == "image/jpeg" {
        bot.send_photo(
            msg.chat.id,
            InputFile::memory(bytes).file_name("preview.jpg"),
        )
        .send()
        .await?;
    } else if content_type == "video/mpeg" || content_type == "video/mp4" {
        bot.send_video(
            msg.chat.id,
            InputFile::memory(bytes).file_name(if content_type == "video/mpeg" {
                "preview.mpeg"
            } else {
                "preview.mp4"
            }),
        )
        .send()
        .await?;
    } else {
        // last effort to see if it's an image
        let img = match image::guess_format(&bytes.to_vec()) {
            Ok(img) => img,
            Err(_) => {
                bot.send_message(msg.chat.id, "Could not get preview :(")
                    .send()
                    .await?;

                return Ok(());
            }
        };

        match img {
            ImageFormat::Png => {
                bot.send_photo(
                    msg.chat.id,
                    InputFile::memory(bytes).file_name("preview.png"),
                )
                .send()
                .await?;
            }
            ImageFormat::Jpeg => {
                bot.send_photo(
                    msg.chat.id,
                    InputFile::memory(bytes).file_name("preview.jpg"),
                )
                .send()
                .await?;
            }
            ImageFormat::Gif => {
                bot.send_photo(
                    msg.chat.id,
                    InputFile::memory(bytes).file_name("preview.gif"),
                )
                .send()
                .await?;
            }
            _ => {
                bot.send_message(msg.chat.id, "Could not get preview :(")
                    .send()
                    .await?;
            }
        }
    }

    bot.delete_message(sending_message.chat.id, sending_message.id)
        .send()
        .await?;

    Ok(())
}
