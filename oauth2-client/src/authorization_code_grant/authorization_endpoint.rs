use http_api_endpoint::{Body, Request, Response};
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
    types::{Scope, State},
};
use serde_json::{Map, Value};
use serde_qs::Error as SerdeQsError;

use crate::ProviderExtAuthorizationCodeGrant;

//
//
//
pub fn render_request<'a, SCOPE>(
    provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
    scopes: impl Into<Option<Vec<SCOPE>>>,
    state: impl Into<Option<State>>,
) -> Result<Request<Body>, AuthorizationEndpointError>
where
    SCOPE: Scope + Serialize,
{
    let mut query = REQ_Query::new(
        provider
            .client_id()
            .cloned()
            .ok_or_else(|| AuthorizationEndpointError::ClientIdMissing)?,
        provider.redirect_uri().map(|x| x.url().to_owned()),
        scopes.into().map(Into::into),
        state.into(),
    );
    if let Some(extensions) = provider.authorization_request_query_extensions() {
        query.set_extensions(extensions);
    }

    let query_str = if let Some(query_str_ret) =
        provider.authorization_request_query_serializing(&query)
    {
        query_str_ret.map_err(|err| {
            AuthorizationEndpointError::CustomSerRequestQueryFailed(err.to_string())
        })?
    } else {
        serde_qs::to_string(&query).map_err(AuthorizationEndpointError::SerRequestQueryFailed)?
    };

    let mut url = provider.authorization_endpoint_url().to_owned();
    url.set_query(Some(query_str.as_str()));

    //
    provider.authorization_request_url_modifying(&mut url);

    //
    let request = Request::builder()
        .method(REQ_METHOD)
        .uri(url.as_str())
        .body(vec![])
        .map_err(AuthorizationEndpointError::MakeRequestFailed)?;

    Ok(request)
}

pub fn parse_response<'a, SCOPE>(
    _provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
    _response: Response<Body>,
) -> Result<(), AuthorizationEndpointError>
where
    SCOPE: Scope + Serialize,
{
    unreachable!()
}

#[derive(thiserror::Error, Debug)]
pub enum AuthorizationEndpointError {
    #[error("ClientIdMissing")]
    ClientIdMissing,
    //
    #[error("CustomSerRequestQueryFailed {0}")]
    CustomSerRequestQueryFailed(String),
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

    return Ok(Err(query));
}

pub type ParseRedirectUriQueryError = SerdeQsError;
