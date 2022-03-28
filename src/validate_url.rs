use url::{ParseError, Url};

pub fn validate_url(url: &str) -> Result<Url, ()> {
    let mut url = Url::parse(url).map_err(|_| ())?;
    url.set_query(None);

    if let None = url.host() {
        return Err(())
    } else if let Some(host) = url.host() {
        if !host.to_string().contains("reddit.com") {
            return Err(())
        }
    }

    Ok(url)
}