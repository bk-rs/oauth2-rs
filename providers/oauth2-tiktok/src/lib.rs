use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://open-api.tiktok.com/oauth/access_token/";
pub const AUTHORIZATION_URL: &str = "https://www.tiktok.com/auth/authorize/";

pub mod authorization_code_grant;

pub use authorization_code_grant::TiktokProviderWithWebApplication;

pub mod extensions;
pub use extensions::TiktokExtensionsBuilder;

// https://developers.tiktok.com/doc/tiktok-api-scopes/
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TiktokScope {
    #[serde(rename = "user.info.basic")]
    UserInfoBasic,
    #[serde(rename = "user.info.email")]
    UserInfoEmail,
    #[serde(rename = "video.list")]
    VideoList,
    #[serde(rename = "video.upload")]
    VideoUpload,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for TiktokScope {}
