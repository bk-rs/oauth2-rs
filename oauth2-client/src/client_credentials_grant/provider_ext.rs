use dyn_clone::{clone_trait_object, DynClone};
pub use oauth2_core::access_token_request::BodyWithClientCredentialsGrant;

use crate::{
    re_exports::{Body, ClientId, ClientSecret, Map, Request, Scope, Url, Value},
    Provider,
};

//
pub trait ProviderExtClientCredentialsGrant: Provider + DynClone {
    fn client_password_in_request_body(&self) -> bool {
        false
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        None
    }

    fn access_token_request_body_extra(
        &self,
        _body: &BodyWithClientCredentialsGrant<<Self as Provider>::Scope>,
    ) -> Option<Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync + 'static>>>
    {
        None
    }

    fn access_token_request_url_modifying(&self, _url: &mut Url) {}

    fn access_token_request_modifying(&self, _request: &mut Request<Body>) {}
}

clone_trait_object!(<SCOPE> ProviderExtClientCredentialsGrant<Scope = SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> core::fmt::Debug for dyn ProviderExtClientCredentialsGrant<Scope = SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProviderExtClientCredentialsGrant")
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
pub struct ProviderExtClientCredentialsGrantStringScopeWrapper<P>
where
    P: ProviderExtClientCredentialsGrant,
{
    inner: P,
}

impl<P> ProviderExtClientCredentialsGrantStringScopeWrapper<P>
where
    P: ProviderExtClientCredentialsGrant,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderExtClientCredentialsGrantStringScopeWrapper<P>
where
    P: ProviderExtClientCredentialsGrant + Clone,
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

impl<P> ProviderExtClientCredentialsGrant for ProviderExtClientCredentialsGrantStringScopeWrapper<P>
where
    P: ProviderExtClientCredentialsGrant + Clone,
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
        body: &BodyWithClientCredentialsGrant<<Self as Provider>::Scope>,
    ) -> Option<Result<Map<String, Value>, Box<dyn std::error::Error + Send + Sync + 'static>>>
    {
        let body =
            match BodyWithClientCredentialsGrant::<<P as Provider>::Scope>::try_from_t_with_string(
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
