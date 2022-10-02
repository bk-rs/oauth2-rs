use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://openapi.baidu.com/oauth/2.0/token";
pub const AUTHORIZATION_URL: &str = "http://openapi.baidu.com/oauth/2.0/authorize";
pub const DEVICE_AUTHORIZATION_URL: &str = "https://openapi.baidu.com/oauth/2.0/device/code";

pub mod authorization_code_grant;
pub mod device_authorization_grant;

pub use authorization_code_grant::BaiduProviderWithWebApplication;
pub use device_authorization_grant::BaiduProviderWithDevice;

pub mod extensions;
pub use extensions::BaiduExtensionsBuilder;

// https://developer.baidu.com/wiki/index.php?title=docs/oauth
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum BaiduScope {
    //
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "netdisk")]
    Netdisk,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for BaiduScope {}
