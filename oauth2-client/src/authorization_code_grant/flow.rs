use std::{error, fmt, str};

use http_api_client::{Client, ClientRespondEndpointError};
use http_api_endpoint::Endpoint;
use oauth2_core::{
    authorization_code_grant::{
        access_token_response::{
            ErrorBody as AT_RES_ErrorBody, SuccessfulBody as AT_RES_SuccessfulBody,
        },
        authorization_response::ErrorQuery as A_RES_ErrorQuery,
    },
    types::State,
};
use serde::{de::DeserializeOwned, Serialize};
use url::{ParseError as UrlParseError, Url};

use crate::{Provider, ProviderExtAuthorizationCodeGrant};

use super::{
    parse_redirect_uri_query, AccessTokenEndpoint, AccessTokenEndpointError, AuthorizationEndpoint,
    AuthorizationEndpointError, AuthorizationEndpointWithDynProvider, ParseRedirectUriQueryError,
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
    pub fn build_authorization_url<P>(
        &self,
        provider: &'a P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError>
    where
        P: ProviderExtAuthorizationCodeGrant,
        <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
        <P as Provider>::Scope: Serialize,
    {
        // Step 1
        build_authorization_url(provider, scopes, state)
    }

    pub fn build_authorization_url_with_dyn_provider(
        &self,
        provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = String>,
        scopes: impl Into<Option<Vec<String>>>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError> {
        // Step 1
        build_authorization_url_with_dyn_provider(provider, scopes, state)
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

        let access_token_endpoint = AccessTokenEndpoint::new(provider, query.code);

        let access_token_ret = self
            .client_with_token
            .respond_endpoint(&access_token_endpoint)
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    FlowHandleCallbackError::AccessTokenEndpointRespondFailed(Box::new(err))
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => {
                    FlowHandleCallbackError::AccessTokenEndpointError(err)
                }
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => {
                    FlowHandleCallbackError::AccessTokenEndpointError(err)
                }
            })?;

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
pub fn build_authorization_url<'a, P>(
    provider: &'a P,
    scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
    state: impl Into<Option<State>>,
) -> Result<Url, FlowBuildAuthorizationUrlError>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: Serialize,
{
    let scopes = scopes.into().or(provider.scopes_default());

    let authorization_endpoint = AuthorizationEndpoint::new(provider, scopes, state);

    let authorization_endpoint_request = authorization_endpoint
        .render_request()
        .map_err(FlowBuildAuthorizationUrlError::AuthorizationEndpointError)?;

    let url = authorization_endpoint_request.uri();

    let url = Url::parse(url.to_string().as_str())
        .map_err(FlowBuildAuthorizationUrlError::ToUrlFailed)?;

    Ok(url)
}

pub fn build_authorization_url_with_dyn_provider<'a>(
    provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = String>,
    scopes: impl Into<Option<Vec<String>>>,
    state: impl Into<Option<State>>,
) -> Result<Url, FlowBuildAuthorizationUrlError> {
    let scopes = scopes.into().or(provider.scopes_default());

    let authorization_endpoint = AuthorizationEndpointWithDynProvider::new(provider, scopes, state);

    let authorization_endpoint_request = authorization_endpoint
        .render_request()
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
