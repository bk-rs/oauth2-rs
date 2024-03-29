use dyn_clone::{clone_trait_object, DynClone};
pub use oauth2_core::access_token_request::BodyWithResourceOwnerPasswordCredentialsGrant;

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

    fn access_token_request_body_extra(
        &self,
        _body: &BodyWithResourceOwnerPasswordCredentialsGrant<<Self as Provider>::Scope>,
    ) -> Option<Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync + 'static>>>
    {
        None
    }
}

clone_trait_object!(<SCOPE> ProviderExtResourceOwnerPasswordCredentialsGrant<Scope = SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> core::fmt::Debug
    for dyn ProviderExtResourceOwnerPasswordCredentialsGrant<Scope = SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

    fn extra(&self) -> Option<Map<String, Value>> {
        self.inner.extra()
    }

    // Note
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

    fn access_token_request_body_extra(
        &self,
        body: &BodyWithResourceOwnerPasswordCredentialsGrant<<Self as Provider>::Scope>,
    ) -> Option<Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync + 'static>>>
    {
        let body =
            match BodyWithResourceOwnerPasswordCredentialsGrant::<<P as Provider>::Scope>::try_from_t_with_string(
                body,
            ) {
                Ok(x) => x,
                Err(err) => return Some(Err(Box::new(err))),
            };

        self.inner.access_token_request_body_extra(&body)
    }

    // Note
}
