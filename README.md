# উড়াধুরা (Uradhura)

![language-rust](https://img.shields.io/badge/Language-Rust-orange)
![language-rust](https://img.shields.io/badge/Platform-Telegram-blue)

Uradhura is a telegram bot which has only one purpose in it's life - fetch info from Reddit posts.

## Features
- Written in Rust. Why not?
- Supports gif, image and even video

## Using Uradhura in Telegram

To begin, add [uradhura_bot](https://t.me/uradhura_bot) to your group, or simply start a private chat with it. ~~It's always a good practice to start a conversation with random cat facts.~~ Use `/rdl <reddit_post_link>` or send a link of a reddit post in the chat and reply it with `/rdl` to fetch media attached to the post of the given link.

## Permissions

Admin privillege with **only "Delete Message" privillege** is required for the bot to function properly.

![image](https://user-images.githubusercontent.com/63340482/160439196-f7aedae9-4b4d-4e59-9c94-e66e6eb986a4.png)


# Build instruction

### Dependencies
- ffmpeg
- openssl
- rust (For compiling)

### Start building

1. Install rustup
2. Install stable toolchain of Rust
3. Run `cargo build --release`
4. Add bot token to `TELOXIDE_TOKEN` environment variable
4. Run the executable in `target/release/uradhura`

# Author

MD Gaziur Rahman - [Telegram](https://t.me/mdgaziur001)
