use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://api.twitter.com/2/oauth2/token";
pub const AUTHORIZATION_URL: &str = "https://twitter.com/i/oauth2/authorize";

pub mod authorization_code_grant;

pub use authorization_code_grant::TwitterProviderWithWebApplication;

pub mod extensions;
pub use extensions::TwitterExtensionsBuilder;

// https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum TwitterScope {
    #[serde(rename = "tweet.read")]
    TweetRead,
    #[serde(rename = "tweet.write")]
    TweetWrite,
    #[serde(rename = "users.read")]
    UsersRead,
    #[serde(rename = "offline.access")]
    OfflineAccess,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for TwitterScope {}
