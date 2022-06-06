use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const AUTHORIZATION_SERVER_ID_DEFAULT: &str = "default";

pub mod authorization_code_grant;

pub use authorization_code_grant::OktaProviderForWebApplication;

pub fn token_url(
    domain: impl AsRef<str>,
    authorization_server_id: impl Into<Option<String>>,
) -> String {
    format!(
        "https://{}/oauth2/{}/v1/token",
        domain.as_ref(),
        authorization_server_id
            .into()
            .unwrap_or_else(|| AUTHORIZATION_SERVER_ID_DEFAULT.to_owned())
    )
}
pub fn authorization_url(
    domain: impl AsRef<str>,
    authorization_server_id: impl Into<Option<String>>,
) -> String {
    format!(
        "https://{}/oauth2/{}/v1/authorize",
        domain.as_ref(),
        authorization_server_id
            .into()
            .unwrap_or_else(|| AUTHORIZATION_SERVER_ID_DEFAULT.to_owned())
    )
}

// Ref https://developer.okta.com/docs/reference/api/oidc/#access-token-scopes-and-claims
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum OktaScope {
    //
    #[serde(rename = "openid")]
    Openid,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "address")]
    Address,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "offline_access")]
    OfflineAccess,
    #[serde(rename = "groups")]
    Groups,
    //
    // TODO
    //
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for OktaScope {}
