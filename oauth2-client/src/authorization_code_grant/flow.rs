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
    types::{Code, CodeChallenge, CodeChallengeMethod, CodeVerifier, Nonce, Scope, State},
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
        config: impl Into<Option<FlowBuildAuthorizationUrlConfiguration>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError>
    where
        SCOPE: Scope + Serialize,
    {
        // Step 1
        build_authorization_url(provider, scopes, config)
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
        config: impl Into<Option<FlowHandleCallbackByQueryConfiguration>>,
    ) -> Result<AT_RES_SuccessfulBody<SCOPE>, FlowHandleCallbackError>
    where
        SCOPE: Scope + Serialize + DeserializeOwned + Send + Sync,
    {
        // Step 3
        let query = parse_redirect_uri_query(query.as_ref())
            .map_err(FlowHandleCallbackError::ParseRedirectUriQueryError)?;

        let query = query.map_err(FlowHandleCallbackError::AuthorizationFailed)?;

        let config: FlowHandleCallbackByQueryConfiguration = config.into().unwrap_or_default();

        if let Some(ref state) = &config.state {
            if let Some(query_state) = &query.state {
                if state != query_state {
                    return Err(FlowHandleCallbackError::StateMismatch);
                }
            } else {
                return Err(FlowHandleCallbackError::StateMissing);
            }
        }

        self.handle_callback(provider, query.code, Some(config.into()))
            .await
    }

    pub async fn handle_callback<SCOPE>(
        &self,
        provider: &(dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync),
        code: Code,
        config: impl Into<Option<FlowHandleCallbackConfiguration>>,
    ) -> Result<AT_RES_SuccessfulBody<SCOPE>, FlowHandleCallbackError>
    where
        SCOPE: Scope + Serialize + DeserializeOwned + Send + Sync,
    {
        let config: FlowHandleCallbackConfiguration = config.into().unwrap_or_default();

        let mut access_token_endpoint = AccessTokenEndpoint::new(provider, code);

        if let Some(code_verifier) = &config.code_verifier {
            access_token_endpoint.set_code_verifier(code_verifier.to_owned());
        }

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
    AccessTokenEndpointRespondFailed(Box<dyn std::error::Error + Send + Sync>),
    #[error("AccessTokenEndpointError {0}")]
    AccessTokenEndpointError(AccessTokenEndpointError),
    #[error("AccessTokenFailed {0:?}")]
    AccessTokenFailed(AT_RES_ErrorBody),
}

//
//
//
#[derive(Debug, Clone, Default)]
pub struct FlowBuildAuthorizationUrlConfiguration {
    pub state: Option<State>,
    pub code_challenge: Option<(CodeChallenge, CodeChallengeMethod)>,
    pub nonce: Option<Nonce>,
}
impl FlowBuildAuthorizationUrlConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }

    pub fn set_state(&mut self, state: State) {
        self.state = Some(state);
    }

    pub fn set_code_challenge(
        &mut self,
        code_challenge: CodeChallenge,
        code_challenge_method: CodeChallengeMethod,
    ) {
        self.code_challenge = Some((code_challenge, code_challenge_method));
    }

    pub fn set_nonce(&mut self, nonce: Nonce) {
        self.nonce = Some(nonce);
    }
}

//
#[derive(Debug, Clone, Default)]
pub struct FlowHandleCallbackByQueryConfiguration {
    pub state: Option<State>,
    pub code_verifier: Option<CodeVerifier>,
}
impl FlowHandleCallbackByQueryConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }

    pub fn set_state(&mut self, state: State) {
        self.state = Some(state);
    }

    pub fn set_code_verifier(&mut self, code_verifier: CodeVerifier) {
        self.code_verifier = Some(code_verifier);
    }
}

//
#[derive(Debug, Clone, Default)]
pub struct FlowHandleCallbackConfiguration {
    pub code_verifier: Option<CodeVerifier>,
}
impl FlowHandleCallbackConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }

    pub fn set_code_verifier(&mut self, code_verifier: CodeVerifier) {
        self.code_verifier = Some(code_verifier);
    }
}

impl From<FlowHandleCallbackByQueryConfiguration> for FlowHandleCallbackConfiguration {
    fn from(c: FlowHandleCallbackByQueryConfiguration) -> Self {
        Self {
            code_verifier: c.code_verifier,
        }
    }
}

//
//
//
pub fn build_authorization_url<SCOPE>(
    provider: &dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
    scopes: impl Into<Option<Vec<SCOPE>>>,
    config: impl Into<Option<FlowBuildAuthorizationUrlConfiguration>>,
) -> Result<Url, FlowBuildAuthorizationUrlError>
where
    SCOPE: Scope + Serialize,
{
    let scopes = scopes.into().or_else(|| provider.scopes_default());

    let config: FlowBuildAuthorizationUrlConfiguration = config.into().unwrap_or_default();

    let mut authorization_endpoint = AuthorizationEndpoint::new(provider, scopes);

    if let Some(state) = &config.state {
        authorization_endpoint.set_state(state.to_owned());
    }

    if let Some((code_challenge, code_challenge_method)) = &config.code_challenge {
        authorization_endpoint
            .set_code_challenge(code_challenge.to_owned(), code_challenge_method.to_owned());
    }

    if let Some(nonce) = &config.nonce {
        authorization_endpoint.set_nonce(nonce.to_owned());
    }

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
