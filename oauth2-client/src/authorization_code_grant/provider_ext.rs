use std::{error, fmt, str};

pub use oauth2_core::{
    access_token_request::BodyWithAuthorizationCodeGrant as AccessTokenRequestBody,
    authorization_code_grant::{
        access_token_response::{
            ErrorBody as AccessTokenResponseErrorBody,
            SuccessfulBody as AccessTokenResponseSuccessfulBody,
        },
        authorization_request::Query as AuthorizationRequestQuery,
    },
    types::ScopeFromStrError,
};

use crate::{
    re_exports::{Body, ClientId, ClientSecret, Map, RedirectUri, Request, Response, Url, Value},
    Provider,
};

//
pub trait ProviderExtAuthorizationCodeGrant: Provider
where
    <<Self as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    fn redirect_uri(&self) -> Option<&RedirectUri>;

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        None
    }

    fn authorization_endpoint_url(&self) -> &Url;

    fn authorization_request_query_extensions(&self) -> Option<Map<String, Value>> {
        None
    }

    fn authorization_request_query_serializing(
        &self,
        _query: &AuthorizationRequestQuery<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn error::Error>>> {
        None
    }

    fn authorization_request_url_modifying(&self, _url: &mut Url) {}

    fn access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        None
    }

    fn access_token_request_rendering(
        &self,
        _body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error>>> {
        None
    }

    fn access_token_response_parsing(
        &self,
        _response: &Response<Body>,
    ) -> Option<
        Result<
            Result<
                AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
                AccessTokenResponseErrorBody,
            >,
            Box<dyn error::Error>,
        >,
    > {
        None
    }
}

//
//
//
pub struct ProviderExtAuthorizationCodeGrantStringScopeWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    inner: P,
}

impl<P> ProviderExtAuthorizationCodeGrantStringScopeWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderExtAuthorizationCodeGrantStringScopeWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
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

impl<P> ProviderExtAuthorizationCodeGrant for ProviderExtAuthorizationCodeGrantStringScopeWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        self.inner.redirect_uri()
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        self.inner
            .scopes_default()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
    }

    fn authorization_endpoint_url(&self) -> &Url {
        self.inner.authorization_endpoint_url()
    }

    fn authorization_request_query_extensions(&self) -> Option<Map<String, Value>> {
        self.inner.authorization_request_query_extensions()
    }

    fn authorization_request_query_serializing(
        &self,
        query: &AuthorizationRequestQuery<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn error::Error>>> {
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

    fn access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        self.inner.access_token_request_body_extensions()
    }

    fn access_token_request_rendering(
        &self,
        body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error>>> {
        self.inner.access_token_request_rendering(body)
    }

    fn access_token_response_parsing(
        &self,
        response: &Response<Body>,
    ) -> Option<
        Result<
            Result<
                AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
                AccessTokenResponseErrorBody,
            >,
            Box<dyn error::Error>,
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
