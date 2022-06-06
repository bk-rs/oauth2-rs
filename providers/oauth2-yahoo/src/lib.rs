use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://api.login.yahoo.com/oauth2/get_token";
pub const AUTHORIZATION_URL: &str = "https://api.login.yahoo.com/oauth2/request_auth";

pub mod authorization_code_grant;

pub use authorization_code_grant::YahooProviderForWebApps;

pub mod extensions;
pub use extensions::YahooExtensionsBuilder;

/// [Ref](https://developer.yahoo.com/oauth2/guide/yahoo_scopes/)
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum YahooScope {
    //
    #[serde(rename = "openid")]
    Openid,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "email")]
    Email,
    //
    // TODO
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for YahooScope {}
