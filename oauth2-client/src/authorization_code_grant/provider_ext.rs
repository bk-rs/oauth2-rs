use dyn_clone::{clone_trait_object, DynClone};
pub use oauth2_core::{
    access_token_request::BodyWithAuthorizationCodeGrant as AccessTokenRequestBody,
    authorization_code_grant::authorization_request::Query as AuthorizationRequestQuery,
    re_exports::{AccessTokenResponseErrorBody, AccessTokenResponseSuccessfulBody},
    types::ScopeFromStrError,
};

use crate::{
    re_exports::{
        Body, ClientId, ClientSecret, Map, RedirectUri, Request, Response, Scope, Url, Value,
    },
    Provider,
};

//
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProviderExtAuthorizationCodeGrantOidcSupportType {
    No,
    Yes,
    Force,
}
impl Default for ProviderExtAuthorizationCodeGrantOidcSupportType {
    fn default() -> Self {
        Self::No
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProviderExtAuthorizationCodeGrantPkceSupportType {
    No,
    Yes,
}
impl Default for ProviderExtAuthorizationCodeGrantPkceSupportType {
    fn default() -> Self {
        Self::No
    }
}

//
pub trait ProviderExtAuthorizationCodeGrant: Provider + DynClone {
    fn redirect_uri(&self) -> Option<&RedirectUri>;

    fn oidc_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantOidcSupportType> {
        None
    }

    fn pkce_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantPkceSupportType> {
        None
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        None
    }

    fn authorization_endpoint_url(&self) -> &Url;

    fn authorization_request_query_extra(&self) -> Option<Map<String, Value>> {
        None
    }

    fn authorization_request_query_serializing(
        &self,
        _query: &AuthorizationRequestQuery<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>> {
        None
    }

    fn authorization_request_url_modifying(&self, _url: &mut Url) {}

    fn access_token_request_body_extra(
        &self,
        _body: &AccessTokenRequestBody,
    ) -> Option<Map<String, Value>> {
        None
    }

    fn access_token_request_rendering(
        &self,
        _body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn std::error::Error + Send + Sync + 'static>>> {
        None
    }

    #[allow(clippy::type_complexity)]
    fn access_token_response_parsing(
        &self,
        _response: &Response<Body>,
    ) -> Option<
        Result<
            Result<
                AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
                AccessTokenResponseErrorBody,
            >,
            Box<dyn std::error::Error + Send + Sync + 'static>,
        >,
    > {
        None
    }
}

clone_trait_object!(<SCOPE> ProviderExtAuthorizationCodeGrant<Scope = SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> core::fmt::Debug for dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProviderExtAuthorizationCodeGrant")
            .field("client_id", &self.client_id())
            .field("token_endpoint_url", &self.token_endpoint_url().as_str())
            .field("redirect_uri", &self.redirect_uri().map(|x| x.to_string()))
            .field("scopes_default", &self.scopes_default())
            .field(
                "authorization_endpoint_url",
                &self.authorization_endpoint_url().as_str(),
            )
            .finish()
    }
}

//
//
//
#[derive(Debug, Clone)]
pub struct ProviderExtAuthorizationCodeGrantStringScopeWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
{
    inner: P,
}

impl<P> ProviderExtAuthorizationCodeGrantStringScopeWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderExtAuthorizationCodeGrantStringScopeWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant + Clone,
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

impl<P> ProviderExtAuthorizationCodeGrant for ProviderExtAuthorizationCodeGrantStringScopeWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant + Clone,
{
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        self.inner.redirect_uri()
    }

    fn oidc_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantOidcSupportType> {
        self.inner.oidc_support_type()
    }

    fn pkce_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantPkceSupportType> {
        self.inner.pkce_support_type()
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        self.inner
            .scopes_default()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
    }

    fn authorization_endpoint_url(&self) -> &Url {
        self.inner.authorization_endpoint_url()
    }

    fn authorization_request_query_extra(&self) -> Option<Map<String, Value>> {
        self.inner.authorization_request_query_extra()
    }

    fn authorization_request_query_serializing(
        &self,
        query: &AuthorizationRequestQuery<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>> {
        let query =
            match AuthorizationRequestQuery::<<P as Provider>::Scope>::try_from_t_with_string(query)
            {
                Ok(x) => x,
                Err(err) => return Some(Err(Box::new(err))),
            };

        self.inner.authorization_request_query_serializing(&query)
    }

    fn authorization_request_url_modifying(&self, url: &mut Url) {
        self.inner.authorization_request_url_modifying(url)
    }

    fn access_token_request_body_extra(
        &self,
        body: &AccessTokenRequestBody,
    ) -> Option<Map<String, Value>> {
        self.inner.access_token_request_body_extra(body)
    }

    fn access_token_request_rendering(
        &self,
        body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn std::error::Error + Send + Sync + 'static>>> {
        self.inner.access_token_request_rendering(body)
    }

    #[allow(clippy::type_complexity)]
    fn access_token_response_parsing(
        &self,
        response: &Response<Body>,
    ) -> Option<
        Result<
            Result<
                AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
                AccessTokenResponseErrorBody,
            >,
            Box<dyn std::error::Error + Send + Sync + 'static>,
        >,
    > {
        self.inner.access_token_response_parsing(response).map(|x| {
            x.map(|y| {
                y.map(|z| AccessTokenResponseSuccessfulBody::<<Self as Provider>::Scope>::from(&z))
            })
        })
    }

    // Note
}
