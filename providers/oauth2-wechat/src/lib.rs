use oauth2_client::provider::{
    serde_enum_str::{Deserialize_enum_str, Serialize_enum_str},
    Scope,
};

pub const TOKEN_URL: &str = "https://api.weixin.qq.com/sns/oauth2/access_token";
pub const AUTHORIZATION_URL: &str = "https://open.weixin.qq.com/connect/qrconnect";

#[cfg(feature = "with-authorization-code-grant")]
pub mod authorization_code_grant;

#[cfg(feature = "with-authorization-code-grant")]
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
