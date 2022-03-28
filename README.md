# uradhura

![language-rust](https://img.shields.io/badge/language-rust-orange)
![language-rust](https://img.shields.io/badge/platform-telgram-blue)

Telegram bot that fetches information from Reddit posts(with gif, image and video support) written in Rust. 

## Using Uradhura in Telegram

Add [bot](https://t.me/uradhura_bot) to your group, or start a private chat. Use /rdl \<reddit link\> or send a link in the chat and reply with /rdl to fetch media. 

### Permissions needed

Give bot admin privillege with only "Delete Message" privillege.

![image](https://user-images.githubusercontent.com/63340482/160439196-f7aedae9-4b4d-4e59-9c94-e66e6eb986a4.png)


## Build instruction

**Dependencies**
1. ffmpeg
2. openssl
3. rust(for compiling)

**Start building**

1. Install rustup
2. Install stable toolchain of Rust
3. Run `cargo build --release`
4. Add bot token to `TELOXIDE_TOKEN` environment variable
4. Run the executable in `target/release/uradhura`

## Author

MD Gaziur Rahman - [Telegram](https://t.me/mdgaziur001)
