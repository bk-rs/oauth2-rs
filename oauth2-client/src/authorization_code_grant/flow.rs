use std::{fmt, str};

use http_api_client::Client;
use http_api_endpoint::Endpoint;
use oauth2_core::{
    authorization_code_grant::authorization_request::State, Provider,
    ProviderExtAuthorizationCodeGrant,
};
use serde::Serialize;
use url::{ParseError as UrlParseError, Url};

use super::{AuthorizationEndpoint, AuthorizationEndpointError};

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
