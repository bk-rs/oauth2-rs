use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://appleid.apple.com/auth/token";
pub const AUTHORIZATION_URL: &str = "https://appleid.apple.com/auth/authorize";

pub mod authorization_code_grant;

pub use authorization_code_grant::AppleProviderWithAppleJs;

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum AppleScope {
    //
    #[serde(rename = "name")]
    Name,
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
impl Scope for AppleScope {}
