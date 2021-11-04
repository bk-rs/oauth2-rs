use std::{error, fmt, str};

use http_api_client::Client;
use oauth2_core::{
    authorization_code_grant::{
        access_token_response::{
            ErrorBody as AT_RES_ErrorBody, SuccessfulBody as AT_RES_SuccessfulBody,
        },
        authorization_response::ErrorQuery as A_RES_ErrorQuery,
    },
    types::{Scope, State},
};
use serde::{de::DeserializeOwned, Serialize};
use url::{ParseError as UrlParseError, Url};

use crate::{Provider, ProviderExtAuthorizationCodeGrant};

use super::{
    access_token_endpoint, authorization_endpoint, parse_redirect_uri_query,
    AccessTokenEndpointError, AuthorizationEndpointError, ParseRedirectUriQueryError,
};

#[derive(Debug, Clone)]
pub struct Flow<C>
where
    C: Client,
{
    client_with_token: C,
}
impl<C> Flow<C>
where
    C: Client,
{
    pub fn new(client_with_token: C) -> Self {
        Self { client_with_token }
    }
}

impl<'a, C> Flow<C>
where
    C: Client,
{
    pub fn build_authorization_url<SCOPE>(
        &self,
        provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
        scopes: impl Into<Option<Vec<SCOPE>>>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError>
    where
        SCOPE: Scope,
        <SCOPE as str::FromStr>::Err: fmt::Display,
        SCOPE: Serialize,
    {
        // Step 1
        build_authorization_url(provider, scopes, state)
    }
}

impl<'a, C> Flow<C>
where
    C: Client + Send + Sync,
{
    pub async fn handle_callback<P>(
        &self,
        provider: &'a P,
        query: impl AsRef<str>,
        state: impl Into<Option<State>>,
    ) -> Result<AT_RES_SuccessfulBody<<P as Provider>::Scope>, FlowHandleCallbackError>
    where
        P: ProviderExtAuthorizationCodeGrant + Send + Sync,
        <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
        <P as Provider>::Scope: DeserializeOwned + Send + Sync,
    {
        // Step 3
        let query = parse_redirect_uri_query(query.as_ref())
            .map_err(FlowHandleCallbackError::ParseRedirectUriQueryError)?;

        let query = query.map_err(FlowHandleCallbackError::AuthorizationFailed)?;

        if let Some(ref state) = state.into() {
            if let Some(query_state) = &query.state {
                if state != query_state {
                    return Err(FlowHandleCallbackError::StateMismatch);
                }
            } else {
                return Err(FlowHandleCallbackError::StateMissing);
            }
        }

        let access_token_endpoint_request =
            access_token_endpoint::render_request(provider, query.code)
                .map_err(FlowHandleCallbackError::AccessTokenEndpointError)?;

        let access_token_endpoint_response = self
            .client_with_token
            .respond(access_token_endpoint_request)
            .await
            .map_err(|err| {
                FlowHandleCallbackError::AccessTokenEndpointRespondFailed(Box::new(err))
            })?;

        let access_token_ret =
            access_token_endpoint::parse_response(provider, access_token_endpoint_response)
                .map_err(FlowHandleCallbackError::AccessTokenEndpointError)?;

        let access_token_successful_body =
            access_token_ret.map_err(FlowHandleCallbackError::AccessTokenFailed)?;

        Ok(access_token_successful_body)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FlowHandleCallbackError {
    #[error("ParseRedirectUriQueryError {0}")]
    ParseRedirectUriQueryError(ParseRedirectUriQueryError),
    //
    #[error("AuthorizationFailed {0:?}")]
    AuthorizationFailed(A_RES_ErrorQuery),
    #[error("StateMismatch")]
    StateMismatch,
    #[error("StateMissing")]
    StateMissing,
    //
    #[error("AccessTokenEndpointRespondFailed {0}")]
    AccessTokenEndpointRespondFailed(Box<dyn error::Error>),
    #[error("AccessTokenEndpointError {0}")]
    AccessTokenEndpointError(AccessTokenEndpointError),
    #[error("AccessTokenFailed {0:?}")]
    AccessTokenFailed(AT_RES_ErrorBody),
}

//
//
//
pub fn build_authorization_url<'a, SCOPE>(
    provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
    scopes: impl Into<Option<Vec<SCOPE>>>,
    state: impl Into<Option<State>>,
) -> Result<Url, FlowBuildAuthorizationUrlError>
where
    SCOPE: Scope + Serialize,
    <SCOPE as str::FromStr>::Err: fmt::Display,
{
    let scopes = scopes.into().or(provider.scopes_default());

    let authorization_endpoint_request =
        authorization_endpoint::render_request(provider, scopes, state)
            .map_err(FlowBuildAuthorizationUrlError::AuthorizationEndpointError)?;

    let url = authorization_endpoint_request.uri();

    let url = Url::parse(url.to_string().as_str())
        .map_err(FlowBuildAuthorizationUrlError::ToUrlFailed)?;

    Ok(url)
}

#[derive(thiserror::Error, Debug)]
pub enum FlowBuildAuthorizationUrlError {
    #[error("AuthorizationEndpointError {0}")]
    AuthorizationEndpointError(AuthorizationEndpointError),
    #[error("ToUrlFailed {0}")]
    ToUrlFailed(UrlParseError),
}
