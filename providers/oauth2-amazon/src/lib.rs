use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL_NA: &str = "https://api.amazon.com/auth/o2/token";
pub const TOKEN_URL_EU: &str = "https://api.amazon.co.uk/auth/o2/token";
pub const TOKEN_URL_FE: &str = "https://api.amazon.co.jp/auth/o2/token";
pub const AUTHORIZATION_URL: &str = "https://www.amazon.com/ap/oa";
pub const DEVICE_AUTHORIZATION_URL: &str = "https://api.amazon.com/auth/o2/create/codepair";

pub mod authorization_code_grant;
pub mod device_authorization_grant;

pub use authorization_code_grant::AmazonProviderWithWebServices;
pub use device_authorization_grant::AmazonProviderWithDevices;

pub mod extensions;
pub use extensions::AmazonExtensionsBuilder;

#[derive(Debug, Clone, PartialEq)]
pub enum AmazonTokenUrlRegion {
    NA,
    EU,
    FE,
}

pub fn token_url(region: impl Into<Option<AmazonTokenUrlRegion>>) -> &'static str {
    match region.into() {
        Some(AmazonTokenUrlRegion::NA) | None => TOKEN_URL_NA,
        Some(AmazonTokenUrlRegion::EU) => TOKEN_URL_EU,
        Some(AmazonTokenUrlRegion::FE) => TOKEN_URL_FE,
    }
}

// Ref https://developer.amazon.com/docs/login-with-amazon/customer-profile.html
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum AmazonScope {
    //
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "profile:user_id")]
    ProfileUserId,
    #[serde(rename = "postal_code")]
    PostalCode,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for AmazonScope {}
