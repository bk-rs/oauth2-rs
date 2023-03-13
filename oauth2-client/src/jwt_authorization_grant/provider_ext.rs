use dyn_clone::{clone_trait_object, DynClone};
pub use oauth2_core::access_token_request::BodyWithJwtAuthorizationGrant;

use crate::{
    re_exports::{Body, ClientId, ClientSecret, Map, Request, Scope, Url, Value},
    Provider,
};

//
pub trait ProviderExtJwtAuthorizationGrant: Provider + DynClone {
    fn assertion(&self) -> &str;

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        None
    }

    fn access_token_request_body_extra(
        &self,
        _body: &BodyWithJwtAuthorizationGrant<<Self as Provider>::Scope>,
    ) -> Option<Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync + 'static>>>
    {
        None
    }

    fn access_token_request_url_modifying(&self, _url: &mut Url) {}

    fn access_token_request_modifying(&self, _request: &mut Request<Body>) {}
}

clone_trait_object!(<SCOPE> ProviderExtJwtAuthorizationGrant<Scope = SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> core::fmt::Debug for dyn ProviderExtJwtAuthorizationGrant<Scope = SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProviderExtJwtAuthorizationGrant")
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
pub struct ProviderExtJwtAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtJwtAuthorizationGrant,
{
    inner: P,
}

impl<P> ProviderExtJwtAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtJwtAuthorizationGrant,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderExtJwtAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtJwtAuthorizationGrant + Clone,
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

impl<P> ProviderExtJwtAuthorizationGrant for ProviderExtJwtAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtJwtAuthorizationGrant + Clone,
{
    fn assertion(&self) -> &str {
        self.inner.assertion()
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        self.inner
            .scopes_default()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
    }

    fn access_token_request_body_extra(
        &self,
        body: &BodyWithJwtAuthorizationGrant<<Self as Provider>::Scope>,
    ) -> Option<Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync + 'static>>>
    {
        let body =
            match BodyWithJwtAuthorizationGrant::<<P as Provider>::Scope>::try_from_t_with_string(
                body,
            ) {
                Ok(x) => x,
                Err(err) => return Some(Err(Box::new(err))),
            };

        self.inner.access_token_request_body_extra(&body)
    }

    fn access_token_request_url_modifying(&self, url: &mut Url) {
        self.inner.access_token_request_url_modifying(url)
    }

    fn access_token_request_modifying(&self, request: &mut Request<Body>) {
        self.inner.access_token_request_modifying(request)
    }
    // Note
}
