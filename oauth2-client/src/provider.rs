use crate::re_exports::{ClientId, ClientSecret, Scope, Url};

//
pub trait Provider {
    type Scope: Scope;

    fn client_id(&self) -> Option<&ClientId>;

    fn client_secret(&self) -> Option<&ClientSecret>;

    fn token_endpoint_url(&self) -> &Url;
}

//
//
//
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
    P: Provider,
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
