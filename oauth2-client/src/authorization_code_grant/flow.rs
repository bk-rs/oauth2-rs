use std::{error, fmt, str};

use http_api_client::{Client, ClientRespondEndpointError};
use http_api_endpoint::Endpoint;
use oauth2_core::{
    authorization_code_grant::{
        access_token_response::{
            ErrorBody as AT_RES_ErrorBody, SuccessfulBody as AT_RES_SuccessfulBody,
        },
        authorization_request::State,
        authorization_response::ErrorQuery as A_RES_ErrorQuery,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::{de::DeserializeOwned, Serialize};
use url::{ParseError as UrlParseError, Url};

use super::{
    parse_redirect_uri_query, AccessTokenEndpoint, AccessTokenEndpointError, AuthorizationEndpoint,
    AuthorizationEndpointError, ParseRedirectUriQueryError,
};

pub struct Flow<C>
where
    C: Client,
{
    client: C,
}
impl<C> Flow<C>
where
    C: Client,
{
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<'a, C> Flow<C>
where
    C: Client + Send + Sync,
{
    pub fn build_authorization_url<P>(
        &self,
        provider: &'a P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError>
    where
        P: ProviderExtAuthorizationCodeGrant + Send + Sync,
        <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
        <P as Provider>::Scope: Serialize + Send + Sync,
    {
        build_authorization_url(provider, scopes, state)
    }

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
        let query = parse_redirect_uri_query(query.as_ref())
            .map_err(FlowHandleCallbackError::ParseRedirectUriQueryError)?;

        let query = query.map_err(FlowHandleCallbackError::AuthorizationFailed)?;

        if state.into() != query.state {
            return Err(FlowHandleCallbackError::StateMismatch);
        }

        let access_token_endpoint = AccessTokenEndpoint::new(provider, query.code);

        let access_token_ret = self
            .client
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
    P: ProviderExtAuthorizationCodeGrant + Send + Sync,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: Serialize + Send + Sync,
{
    let authorization_endpoint = AuthorizationEndpoint::new(provider, scopes, state);

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
