use oauth2_core::{
    provider::{Map, Url, UrlParseError, Value},
    types::{ClientId, ClientSecret},
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{GoogleScope, DEVICE_AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct GoogleProviderForTvAndDeviceApps {
    client_id: ClientId,
    client_secret: ClientSecret,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl GoogleProviderForTvAndDeviceApps {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            token_endpoint_url: TOKEN_URL.parse()?,
            device_authorization_endpoint_url: DEVICE_AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for GoogleProviderForTvAndDeviceApps {
    type Scope = GoogleScope;

    fn client_id(&self) -> Option<&ClientId> {
        Some(&self.client_id)
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        Some(&self.client_secret)
    }

    fn token_endpoint_url(&self) -> &Url {
        &self.token_endpoint_url
    }
}
impl ProviderExtDeviceAuthorizationGrant for GoogleProviderForTvAndDeviceApps {
    fn device_authorization_endpoint_url(&self) -> &Url {
        &self.device_authorization_endpoint_url
    }

    fn device_access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        let mut map = Map::new();
        map.insert(
            "client_secret".to_owned(),
            Value::String(self.client_secret.to_owned()),
        );
        Some(map)
    }
}
