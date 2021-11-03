use oauth2_client::provider::{
    serde_enum_str::{Deserialize_enum_str, Serialize_enum_str},
    Scope,
};

pub const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
pub const AUTHORIZATION_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const DEVICE_AUTHORIZATION_URL: &str = "https://oauth2.googleapis.com/device/code";

#[cfg(feature = "with-authorization-code-grant")]
pub mod authorization_code_grant;
#[cfg(feature = "with-device-authorization-grant")]
pub mod device_authorization_grant;

#[cfg(feature = "with-authorization-code-grant")]
pub use authorization_code_grant::{
    GoogleProviderForWebServerApps, GoogleProviderForWebServerAppsAccessType,
};
#[cfg(feature = "with-device-authorization-grant")]
pub use device_authorization_grant::GoogleProviderForTvAndDeviceApps;

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum GoogleScope {
    //
    #[serde(rename = "email")]
    #[serde(alias = "https://www.googleapis.com/auth/userinfo.email")]
    Email,
    #[serde(rename = "profile")]
    #[serde(alias = "https://www.googleapis.com/auth/userinfo.profile")]
    Profile,
    //
    #[serde(rename = "openid")]
    Openid,
    //
    #[serde(rename = "https://www.googleapis.com/auth/drive.file")]
    DriveFile,
    //
    #[serde(rename = "https://www.googleapis.com/auth/youtube")]
    Youtube,
    #[serde(rename = "https://www.googleapis.com/auth/youtube.readonly")]
    YoutubeReadonly,
    //
    // TODO
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for GoogleScope {}
