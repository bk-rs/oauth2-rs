use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://api.instagram.com/oauth/access_token";
pub const AUTHORIZATION_URL: &str = "https://api.instagram.com/oauth/authorize";

pub mod authorization_code_grant;

pub use authorization_code_grant::InstagramProviderForBasicDisplayApi;

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InstagramScope {
    // Ref https://github.com/bk-rs/instagram-rs/blob/master/instagram-permission/src/lib.rs
    UserProfile,
    UserMedia,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for InstagramScope {}
