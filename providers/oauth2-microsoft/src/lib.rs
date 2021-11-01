use oauth2_core::types::Scope;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[cfg(feature = "with-authorization-code-grant")]
pub mod authorization_code_grant;
#[cfg(feature = "with-device-authorization-grant")]
pub mod device_authorization_grant;

#[cfg(feature = "with-device-authorization-grant")]
pub use device_authorization_grant::MicrosoftProviderWithDevice;

pub fn token_url(tenant: impl AsRef<str>) -> String {
    format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant.as_ref()
    )
}
pub fn device_authorization_url(tenant: impl AsRef<str>) -> String {
    format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/devicecode",
        tenant.as_ref()
    )
}

// Ref https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-permissions-and-consent
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum MicrosoftScope {
    //
    #[serde(rename = "openid")]
    Openid,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "offline_access")]
    OfflineAccess,
    //
    #[serde(rename = "User.Read")]
    #[serde(alias = "https://graph.microsoft.com/User.Read")]
    UserRead,
    //
    // TODO
    //
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for MicrosoftScope {}
