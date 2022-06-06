use oauth2_client::re_exports::{
    Deserialize_enum_str, Scope, Serialize_enum_str, Url, UrlParseError,
};

pub const BASE_URL_GITLAB_COM: &str = "https://gitlab.com/";

pub mod authorization_code_grant;

pub use authorization_code_grant::GitlabProviderForEndUsers;

pub mod extensions;
pub use extensions::GitlabExtensionsBuilder;

pub fn token_url(base_url: impl AsRef<str>) -> Result<Url, UrlParseError> {
    Url::parse(base_url.as_ref())?.join("/oauth/token")
}
pub fn authorization_url(base_url: impl AsRef<str>) -> Result<Url, UrlParseError> {
    Url::parse(base_url.as_ref())?.join("/oauth/authorize")
}

// Ref https://gitlab.com/-/profile/applications
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum GitlabScope {
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "read_user")]
    ReadUser,
    #[serde(rename = "read_api")]
    ReadApi,
    #[serde(rename = "read_repository")]
    ReadRepository,
    #[serde(rename = "write_repository")]
    WriteRepository,
    #[serde(rename = "read_registry")]
    ReadRegistry,
    #[serde(rename = "write_registry")]
    WriteRegistry,
    #[serde(rename = "sudo")]
    Sudo,
    #[serde(rename = "openid")]
    Openid,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "email")]
    Email,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for GitlabScope {}

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
