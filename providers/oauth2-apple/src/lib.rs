use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://appleid.apple.com/auth/token";
pub const AUTHORIZATION_URL: &str = "https://appleid.apple.com/auth/authorize";
pub const OAUTH2_TOKEN_URL: &str = "https://appleid.apple.com/auth/oauth2/token";

pub mod authorization_code_grant;
pub mod client_credentials_grant;

pub use authorization_code_grant::AppleProviderWithAppleJs;
pub use client_credentials_grant::AppleProviderForSearchAdsApi;

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum AppleScope {
    //
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "email")]
    Email,
    //
    // https://developer.apple.com/documentation/apple_search_ads/implementing_oauth_for_the_apple_search_ads_api
    //
    #[serde(rename = "searchadsorg")]
    Searchadsorg,
    //
    // TODO
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for AppleScope {}
