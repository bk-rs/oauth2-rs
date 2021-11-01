use oauth2_core::{
    provider::{Url, UrlParseError},
    types::{ClientId, ClientSecret, Scope},
    Provider, ProviderExtDeviceAuthorizationGrant,
};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
//
//
#[derive(Debug, Clone)]
pub struct MicrosoftProviderWithDevice {
    client_id: ClientId,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl MicrosoftProviderWithDevice {
    pub fn new(tenant: String, client_id: ClientId) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            token_endpoint_url: format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                tenant
            )
            .parse()?,
            device_authorization_endpoint_url: format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/devicecode",
                tenant
            )
            .parse()?,
        })
    }
}
impl Provider for MicrosoftProviderWithDevice {
    type Scope = MicrosoftScope;

    fn client_id(&self) -> Option<ClientId> {
        Some(self.client_id.to_owned())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        None
    }

    fn token_endpoint_url(&self) -> Url {
        self.token_endpoint_url.to_owned()
    }
}
impl ProviderExtDeviceAuthorizationGrant for MicrosoftProviderWithDevice {
    fn device_authorization_endpoint_url(&self) -> Url {
        self.device_authorization_endpoint_url.to_owned()
    }
}

//
//
//
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
