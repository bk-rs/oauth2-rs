use url::Url;

use crate::types::{ClientId, ClientSecret, Scope};

pub trait Provider {
    type Scope: Scope;

    fn client_id(&self) -> Option<ClientId>;

    fn client_secret(&self) -> Option<ClientSecret>;

    fn token_endpoint_url(&self) -> Url;
}

#[cfg(feature = "with-authorization-code-grant")]
pub trait ProviderExtAuthorizationCodeGrant: Provider {
    fn authorization_endpoint_url(&self) -> Url;
}

#[cfg(feature = "with-device-authorization-grant")]
pub trait ProviderExtDeviceAuthorizationGrant: Provider {
    fn device_authorization_endpoint_url(&self) -> Url;
}
