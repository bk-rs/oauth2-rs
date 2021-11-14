use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub mod authorization_code_grant;
pub mod device_authorization_grant;

pub use authorization_code_grant::MicrosoftProviderForWebApps;
pub use device_authorization_grant::MicrosoftProviderForDevices;

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
pub fn authorization_url(tenant: impl AsRef<str>) -> String {
    format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
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
