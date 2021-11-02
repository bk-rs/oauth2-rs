//! https://datatracker.ietf.org/doc/html/rfc6749#section-3.1.2

use url::Url;

#[derive(Debug, Clone)]
pub struct RedirectUri {
    url: Url,
}
impl RedirectUri {
    pub fn new(url: impl AsRef<str>) -> Result<Self, String> {
        let url = url.as_ref().parse::<Url>().map_err(|err| err.to_string())?;

        if !vec!["http", "https"].contains(&url.scheme()) {
            return Err(format!("Invalid scheme: {}", url.scheme()));
        }

        Ok(Self { url })
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}
