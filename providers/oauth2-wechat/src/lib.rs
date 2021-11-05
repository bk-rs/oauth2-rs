use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://api.weixin.qq.com/sns/oauth2/access_token";
pub const AUTHORIZATION_URL: &str = "https://open.weixin.qq.com/connect/qrconnect";

pub mod authorization_code_grant;

pub use authorization_code_grant::WeChatProviderWithWebApplication;

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum WeChatScope {
    //
    #[serde(rename = "snsapi_login")]
    SnsapiLogin,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for WeChatScope {}
