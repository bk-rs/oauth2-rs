use std::error;

use http_api_client::{Client, ClientRespondEndpointError};
use http_api_client_endpoint::Endpoint as _;
use oauth2_core::{
    authorization_code_grant::{
        access_token_response::{
            ErrorBody as AT_RES_ErrorBody, SuccessfulBody as AT_RES_SuccessfulBody,
        },
        authorization_response::ErrorQuery as A_RES_ErrorQuery,
    },
    serde::{de::DeserializeOwned, Serialize},
    types::{Code, Scope, State},
    url::{ParseError as UrlParseError, Url},
};

use crate::ProviderExtAuthorizationCodeGrant;

use super::{
    parse_redirect_uri_query, AccessTokenEndpoint, AccessTokenEndpointError, AuthorizationEndpoint,
    AuthorizationEndpointError, ParseRedirectUriQueryError,
};

//
//
//
#[derive(Debug, Clone)]
pub struct Flow<C>
where
    C: Client,
{
    pub client_with_token: C,
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
    /// Don't require state if for Mobile & Desktop Apps
    pub fn build_authorization_url<SCOPE>(
        &self,
        provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
        scopes: impl Into<Option<Vec<SCOPE>>>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError>
    where
        SCOPE: Scope + Serialize,
    {
        // Step 1
        build_authorization_url(provider, scopes, state, None)
    }

    // OIDC
    pub fn build_authorization_url_with_oidc<SCOPE>(
        &self,
        provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
        scopes: impl Into<Option<Vec<SCOPE>>>,
        state: impl Into<Option<State>>,
        nonce: impl Into<Option<String>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError>
    where
        SCOPE: Scope + Serialize,
    {
        // Step 1
        build_authorization_url(provider, scopes, state, nonce)
    }
}

impl<C> Flow<C>
where
    C: Client + Send + Sync,
{
    pub async fn handle_callback_by_query<SCOPE>(
        &self,
        provider: &(dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync),
        query: impl AsRef<str>,
        state: impl Into<Option<State>>,
    ) -> Result<AT_RES_SuccessfulBody<SCOPE>, FlowHandleCallbackError>
    where
        SCOPE: Scope + Serialize + DeserializeOwned + Send + Sync,
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

        self.handle_callback(provider, query.code).await
    }

    pub async fn handle_callback<SCOPE>(
        &self,
        provider: &(dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync),
        code: Code,
    ) -> Result<AT_RES_SuccessfulBody<SCOPE>, FlowHandleCallbackError>
    where
        SCOPE: Scope + Serialize + DeserializeOwned + Send + Sync,
    {
        let access_token_endpoint = AccessTokenEndpoint::new(provider, code);

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
    AccessTokenEndpointRespondFailed(Box<dyn error::Error + Send + Sync>),
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
    nonce: impl Into<Option<String>>,
) -> Result<Url, FlowBuildAuthorizationUrlError>
where
    SCOPE: Scope + Serialize,
{
    let scopes = scopes.into().or(provider.scopes_default());

    let mut authorization_endpoint = AuthorizationEndpoint::new(provider, scopes, state);
    authorization_endpoint.nonce = nonce.into();

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
