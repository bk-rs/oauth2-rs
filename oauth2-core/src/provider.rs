pub use serde_json::{Map, Value};
pub use url::{ParseError as UrlParseError, Url};

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

    fn device_access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        None
    }
}
