use dyn_clone::{clone_trait_object, DynClone};

use crate::re_exports::{ClientId, ClientSecret, Map, Scope, Url, Value};

//
//
//
pub trait Provider: DynClone {
    type Scope: Scope;

    fn client_id(&self) -> Option<&ClientId>;

    fn client_secret(&self) -> Option<&ClientSecret>;

    fn token_endpoint_url(&self) -> &Url;

    // e.g. Mastodon's base_url
    fn extra(&self) -> Option<Map<String, Value>> {
        None
    }
}

clone_trait_object!(<SCOPE> Provider<Scope = SCOPE> where SCOPE: Scope + Clone);

//
//
//
#[derive(Debug, Clone)]
pub struct ProviderStringScopeWrapper<P>
where
    P: Provider,
{
    inner: P,
}

impl<P> ProviderStringScopeWrapper<P>
where
    P: Provider,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderStringScopeWrapper<P>
where
    P: Provider + Clone,
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
