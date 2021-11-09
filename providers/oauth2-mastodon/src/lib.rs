use oauth2_client::re_exports::{
    Deserialize_enum_str, Scope, Serialize_enum_str, Url, UrlParseError,
};

pub const BASE_URL_MASTODON_SOCIAL: &str = "https://mastodon.social/";

pub mod authorization_code_grant;
pub mod client_credentials_grant;

pub use authorization_code_grant::MastodonProviderForEndUsers;
pub use client_credentials_grant::MastodonProviderForEndApplications;

pub mod additional_endpoints;
pub use additional_endpoints::MastodonEndpointBuilder;

pub fn token_url(base_url: impl AsRef<str>) -> Result<Url, UrlParseError> {
    Ok(Url::parse(base_url.as_ref())?.join("/oauth/token")?)
}
pub fn authorization_url(base_url: impl AsRef<str>) -> Result<Url, UrlParseError> {
    Ok(Url::parse(base_url.as_ref())?.join("/oauth/authorize")?)
}

// Ref https://docs.joinmastodon.org/api/oauth-scopes/
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum MastodonScope {
    //
    //
    //
    #[serde(rename = "read")]
    Read,
    //
    //
    //
    #[serde(rename = "write")]
    Write,
    //
    //
    //
    #[serde(rename = "follow")]
    Follow,
    //
    //
    //
    #[serde(rename = "push")]
    Push,
    //
    //
    //
    #[serde(rename = "admin:read")]
    AdminRead,
    //
    //
    //
    #[serde(rename = "admin:write")]
    AdminWrite,
    //
    //
    //
    #[serde(rename = "crypto")]
    Crypto,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for MastodonScope {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_url() {
        assert_eq!(
            token_url("https://mastodon.social/").unwrap().as_str(),
            "https://mastodon.social/oauth/token"
        );
        assert_eq!(
            token_url("https://mastodon.social").unwrap().as_str(),
            "https://mastodon.social/oauth/token"
        );
    }

    #[test]
    fn test_authorization_url() {
        assert_eq!(
            authorization_url("https://mastodon.social/")
                .unwrap()
                .as_str(),
            "https://mastodon.social/oauth/authorize"
        );
        assert_eq!(
            authorization_url("https://mastodon.social")
                .unwrap()
                .as_str(),
            "https://mastodon.social/oauth/authorize"
        );
    }
}
