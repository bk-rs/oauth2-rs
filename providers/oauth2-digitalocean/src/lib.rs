use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://cloud.digitalocean.com/v1/oauth/token";
pub const AUTHORIZATION_URL: &str = "https://cloud.digitalocean.com/v1/oauth/authorize";

pub mod authorization_code_grant;

pub use authorization_code_grant::DigitaloceanProviderWithWebApplication;

pub mod extensions;
pub use extensions::DigitaloceanExtensionsBuilder;

// Ref https://docs.digitalocean.com/reference/api/oauth-api/#scopes
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum DigitaloceanScope {
    //
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for DigitaloceanScope {}
