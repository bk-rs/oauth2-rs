use oauth2_client::{
    re_exports::{ClientId, ClientSecret, Url, UrlParseError},
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{device_authorization_url, token_url, MicrosoftScope};

#[derive(Debug, Clone)]
pub struct MicrosoftProviderForDevices {
    client_id: ClientId,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl MicrosoftProviderForDevices {
    pub fn new(tenant: impl AsRef<str>, client_id: ClientId) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            token_endpoint_url: token_url(tenant.as_ref()).parse()?,
            device_authorization_endpoint_url: device_authorization_url(tenant.as_ref()).parse()?,
        })
    }
}
impl Provider for MicrosoftProviderForDevices {
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
impl ProviderExtDeviceAuthorizationGrant for MicrosoftProviderForDevices {
    fn device_authorization_endpoint_url(&self) -> &Url {
        &self.device_authorization_endpoint_url
    }
}
