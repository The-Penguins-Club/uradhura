use url::Url;
use teloxide::prelude2::*;

pub async fn validate_url(url: &str) -> Result<Url, ()> {
    let mut url = Url::parse(url).map_err(|_| ())?;
    url.set_query(None);

    if let None = url.host() {
        return Err(())
    } else if let Some(host) = url.clone().host() {
        if host.to_string() == "redd.it" {
            url = reqwest::get(url)
                .await
                .map_err(|_| ())?
                .url()
                .clone();
        }

        if !url.host().unwrap().to_string().contains("reddit.com") {
            dbg!("Wtf!");
            return Err(())
        }
    }

    Ok(url)
}

pub fn get_sender(message: &Message) -> String {
    match message.from() {
        None => "deleted_user".to_string(),
        Some(u) => match &u.username {
            Some(username) => username.to_string(),
            None => {
                u.first_name.clone()
                    + &if let Some(last_name) = &u.last_name {
                    " ".to_string() + last_name
                } else {
                    "".to_string()
                }
            }
        },
    }
}
