use std::{fmt, str};

use crate::{
    re_exports::{ClientId, ClientSecret, Map, Url, Value},
    Provider,
};

//
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

//
//
//
pub struct ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    inner: P,
}

impl<P> ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    type Scope = String;

    fn client_id(&self) -> Option<&ClientId> {
        self.inner.client_id()
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        self.inner.client_secret()
    }

    fn token_endpoint_url(&self) -> &Url {
        self.inner.token_endpoint_url()
    }
}

impl<P> ProviderExtDeviceAuthorizationGrant
    for ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        self.inner
            .scopes_default()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
    }

    fn device_authorization_endpoint_url(&self) -> &Url {
        self.inner.device_authorization_endpoint_url()
    }

    fn device_authorization_request_body_extensions(&self) -> Option<Map<String, Value>> {
        self.inner.device_authorization_request_body_extensions()
    }

    fn device_access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        self.inner.device_access_token_request_body_extensions()
    }

    // Note
}
