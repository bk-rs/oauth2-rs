use oauth2_core::{
    provider::{Url, UrlParseError},
    types::{ClientId, ClientSecret},
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{GithubScope, DEVICE_AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct GithubProviderWithDevice {
    client_id: ClientId,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl GithubProviderWithDevice {
    pub fn new(client_id: ClientId) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            token_endpoint_url: TOKEN_URL.parse()?,
            device_authorization_endpoint_url: DEVICE_AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for GithubProviderWithDevice {
    type Scope = GithubScope;

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
impl ProviderExtDeviceAuthorizationGrant for GithubProviderWithDevice {
    fn device_authorization_endpoint_url(&self) -> Url {
        self.device_authorization_endpoint_url.to_owned()
    }
}
