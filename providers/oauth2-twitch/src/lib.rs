use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";
pub const AUTHORIZATION_URL: &str = "https://id.twitch.tv/oauth2/authorize";

pub mod authorization_code_grant;

pub use authorization_code_grant::TwitchProviderForWebServerApps;

pub mod extensions;
pub use extensions::TwitchExtensionsBuilder;

/// [Ref](https://dev.twitch.tv/docs/authentication#scopes)
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum TwitchScope {
    //
    #[serde(rename = "user:read:email")]
    UserReadEmail,
    //
    // TODO
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for TwitchScope {}
