use std::{convert::Infallible, error};

use http_api_client_endpoint::{Body, Endpoint, Request, Response};
use oauth2_core::{
    access_token_response::GENERAL_ERROR_BODY_KEY_ERROR,
    authorization_code_grant::{
        authorization_request::{Query as REQ_Query, METHOD as REQ_METHOD},
        authorization_response::{
            ErrorQuery as RES_ErrorQuery, SuccessfulQuery as RES_SuccessfulQuery,
        },
    },
    http::Error as HttpError,
    serde::Serialize,
    types::{Nonce, Scope, State},
};
use serde_json::{Map, Value};
use serde_qs::Error as SerdeQsError;

use crate::ProviderExtAuthorizationCodeGrant;

//
//
//
#[derive(Clone)]
pub struct AuthorizationEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
    scopes: Option<Vec<SCOPE>>,
    state: Option<State>,
    pub nonce: Option<Nonce>,
}
impl<'a, SCOPE> AuthorizationEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
        scopes: impl Into<Option<Vec<SCOPE>>>,
        state: impl Into<Option<State>>,
    ) -> Self {
        Self {
            provider,
            scopes: scopes.into(),
            state: state.into(),
            nonce: None,
        }
    }
}

impl<'a, SCOPE> Endpoint for AuthorizationEndpoint<'a, SCOPE>
where
    SCOPE: Scope + Serialize,
{
    type RenderRequestError = AuthorizationEndpointError;

    type ParseResponseOutput = ();
    type ParseResponseError = Infallible;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut query = REQ_Query::new(
            self.provider
                .client_id()
                .cloned()
                .ok_or(AuthorizationEndpointError::ClientIdMissing)?,
            self.provider.redirect_uri().map(|x| x.to_string()),
            self.scopes.to_owned().map(Into::into),
            self.state.to_owned(),
        );
        query.nonce = self.nonce.to_owned();

        if let Some(extra) = self.provider.authorization_request_query_extra() {
            query.set_extra(extra);
        }

        let query_str = if let Some(query_str_ret) = self
            .provider
            .authorization_request_query_serializing(&query)
        {
            query_str_ret
                .map_err(|err| AuthorizationEndpointError::CustomSerRequestQueryFailed(err))?
        } else {
            serde_qs::to_string(&query)
                .map_err(AuthorizationEndpointError::SerRequestQueryFailed)?
        };

        let mut url = self.provider.authorization_endpoint_url().to_owned();
        url.set_query(Some(query_str.as_str()));

        //
        self.provider.authorization_request_url_modifying(&mut url);

        //
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
    //
    #[error("CustomSerRequestQueryFailed {0}")]
    CustomSerRequestQueryFailed(Box<dyn error::Error + Send + Sync>),
    //
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

    Ok(Err(query))
}

pub type ParseRedirectUriQueryError = SerdeQsError;
