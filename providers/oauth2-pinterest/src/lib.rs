use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://api.pinterest.com/v5/oauth/token";
pub const AUTHORIZATION_URL: &str = "https://www.pinterest.com/oauth/";

pub mod authorization_code_grant;

pub use authorization_code_grant::PinterestProviderWithWebApplication;

pub mod extensions;
pub use extensions::PinterestExtensionsBuilder;

//
// https://developers.pinterest.com/docs/getting-started/scopes/#pinterest_oauth2
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PinterestScope {
    #[serde(rename = "boards:read")]
    BoardsRead,
    #[serde(rename = "boards:write")]
    BoardsWrite,
    #[serde(rename = "pins:read")]
    PinsRead,
    #[serde(rename = "pins:write")]
    PinsWrite,
    #[serde(rename = "user_accounts:read")]
    UserAccountsRead,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for PinterestScope {}
