use std::{convert::Infallible, fmt, str};

use http_api_endpoint::{http::Error as HttpError, Body, Endpoint, Request, Response};
use oauth2_core::{
    access_token_response::GENERAL_ERROR_BODY_KEY_ERROR,
    authorization_code_grant::{
        authorization_request::{Query as REQ_Query, State, METHOD as REQ_METHOD},
        authorization_response::{
            ErrorQuery as RES_ErrorQuery, SuccessfulQuery as RES_SuccessfulQuery,
        },
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::Serialize;
use serde_json::{Map, Value};
use serde_qs::Error as SerdeQsError;

//
pub struct AuthorizationEndpoint<'a, P>
where
    P: ProviderExtAuthorizationCodeGrant,
{
    provider: &'a P,
    scopes: Option<Vec<<P as Provider>::Scope>>,
    state: Option<State>,
}
impl<'a, P> AuthorizationEndpoint<'a, P>
where
    P: ProviderExtAuthorizationCodeGrant,
{
    pub fn new(
        provider: &'a P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        state: impl Into<Option<State>>,
    ) -> Self {
        Self {
            provider,
            scopes: scopes.into(),
            state: state.into(),
        }
    }
}

impl<'a, P> Endpoint for AuthorizationEndpoint<'a, P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: Serialize,
{
    type RenderRequestError = AuthorizationEndpointError;

    type ParseResponseOutput = ();
    type ParseResponseError = Infallible;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut query = REQ_Query::new(
            self.provider
                .client_id()
                .ok_or_else(|| AuthorizationEndpointError::ClientIdMissing)?,
            self.provider.redirect_uri().map(|x| x.url().to_owned()),
            self.scopes.to_owned().map(Into::into),
            self.state.to_owned(),
        );
        query._extensions = self.provider.authorization_request_query_extensions();

        let query_str = serde_qs::to_string(&query)
            .map_err(AuthorizationEndpointError::SerRequestQueryFailed)?;

        let mut url = self.provider.authorization_endpoint_url();
        url.set_query(Some(query_str.as_str()));

        let request = Request::builder()
            .method(REQ_METHOD)
            .uri(url.as_str())
            .body(vec![])
            .map_err(AuthorizationEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        _response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        unreachable!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthorizationEndpointError {
    #[error("ClientIdMissing")]
    ClientIdMissing,
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
}

//
//
//
pub fn parse_redirect_uri_query(
    query_str: impl AsRef<str>,
) -> Result<Result<RES_SuccessfulQuery, RES_ErrorQuery>, ParseRedirectUriQueryError> {
    let map = serde_qs::from_str::<Map<String, Value>>(query_str.as_ref())?;
    if !map.contains_key(GENERAL_ERROR_BODY_KEY_ERROR) {
        let query = serde_qs::from_str::<RES_SuccessfulQuery>(query_str.as_ref())?;

        return Ok(Ok(query));
    }

    let query = serde_qs::from_str::<RES_ErrorQuery>(query_str.as_ref())?;

    return Ok(Err(query));
}

pub type ParseRedirectUriQueryError = SerdeQsError;
