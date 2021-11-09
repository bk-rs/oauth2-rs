use std::fmt;

use dyn_clone::{clone_trait_object, DynClone};

use crate::{
    re_exports::{ClientId, ClientSecret, Map, Scope, Url, Value},
    Provider,
};

//
pub trait ProviderExtResourceOwnerPasswordCredentialsGrant: Provider + DynClone {
    fn client_password_in_request_body(&self) -> bool {
        false
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        None
    }

    fn access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        None
    }
}

clone_trait_object!(<SCOPE> ProviderExtResourceOwnerPasswordCredentialsGrant<Scope = SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> fmt::Debug
    for dyn ProviderExtResourceOwnerPasswordCredentialsGrant<Scope = SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProviderExtResourceOwnerPasswordCredentialsGrant")
            .field("client_id", &self.client_id())
            .field("token_endpoint_url", &self.token_endpoint_url().as_str())
            .field("scopes_default", &self.scopes_default())
            .finish()
    }
}

//
//
//
#[derive(Debug, Clone)]
pub struct ProviderExtResourceOwnerPasswordCredentialsGrantStringScopeWrapper<P>
where
    P: ProviderExtResourceOwnerPasswordCredentialsGrant,
{
    inner: P,
}

impl<P> ProviderExtResourceOwnerPasswordCredentialsGrantStringScopeWrapper<P>
where
    P: ProviderExtResourceOwnerPasswordCredentialsGrant,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderExtResourceOwnerPasswordCredentialsGrantStringScopeWrapper<P>
where
    P: ProviderExtResourceOwnerPasswordCredentialsGrant + Clone,
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

impl<P> ProviderExtResourceOwnerPasswordCredentialsGrant
    for ProviderExtResourceOwnerPasswordCredentialsGrantStringScopeWrapper<P>
where
    P: ProviderExtResourceOwnerPasswordCredentialsGrant + Clone,
{
    fn client_password_in_request_body(&self) -> bool {
        self.inner.client_password_in_request_body()
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        self.inner
            .scopes_default()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
    }

    fn access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        self.inner.access_token_request_body_extensions()
    }

    // Note
}
