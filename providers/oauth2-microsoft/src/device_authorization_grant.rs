use oauth2_core::{
    provider::{Url, UrlParseError},
    types::{ClientId, ClientSecret},
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{device_authorization_url, token_url, MicrosoftScope};

#[derive(Debug, Clone)]
pub struct MicrosoftProviderWithDevice {
    client_id: ClientId,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl MicrosoftProviderWithDevice {
    pub fn new(tenant: impl AsRef<str>, client_id: ClientId) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            token_endpoint_url: token_url(tenant.as_ref()).parse()?,
            device_authorization_endpoint_url: device_authorization_url(tenant.as_ref()).parse()?,
        })
    }
}
impl Provider for MicrosoftProviderWithDevice {
    type Scope = MicrosoftScope;

    fn client_id(&self) -> Option<&ClientId> {
        Some(&self.client_id)
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        None
    }

    fn token_endpoint_url(&self) -> &Url {
        &self.token_endpoint_url
    }
}
impl ProviderExtDeviceAuthorizationGrant for MicrosoftProviderWithDevice {
    fn device_authorization_endpoint_url(&self) -> &Url {
        &self.device_authorization_endpoint_url
    }
}
