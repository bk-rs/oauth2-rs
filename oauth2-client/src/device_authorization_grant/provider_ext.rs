use dyn_clone::{clone_trait_object, DynClone};

use crate::{
    re_exports::{ClientId, ClientSecret, Map, Scope, Url, Value},
    Provider,
};

//
pub trait ProviderExtDeviceAuthorizationGrant: Provider + DynClone + Send + Sync {
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

clone_trait_object!(<SCOPE> ProviderExtDeviceAuthorizationGrant<Scope = SCOPE> where SCOPE: Scope + Clone);

//
//
//
#[derive(Debug, Clone)]
pub struct ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
{
    inner: P,
}

impl<P> ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant + Clone,
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
    P: ProviderExtDeviceAuthorizationGrant + Clone,
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
