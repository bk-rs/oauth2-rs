use crate::{
    provider::{Map, Url, Value},
    Provider,
};

pub trait ProviderExtDeviceAuthorizationGrant: Provider {
    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        None
    }

    fn device_authorization_endpoint_url(&self) -> &Url;

    fn device_authorization_request_body_extensions(&self) -> Option<Map<String, Value>> {
        None
    }

    fn device_access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        None
    }
}
