use std::{fmt, str};

use http_api_client::RetryableClient;
use oauth2_core::{
    device_authorization_grant::{
        device_access_token_response::{
            ErrorBody as DATRES_ErrorBody, SuccessfulBody as DATRES_SuccessfulBody,
        },
        device_authorization_response::{
            ErrorBody as DARES_ErrorBody, UserCode, VerificationUri, VerificationUriComplete,
        },
    },
    Provider, ProviderExtDeviceAuthorizationGrant,
};
use serde::{de::DeserializeOwned, Serialize};

use super::{
    DeviceAccessTokenEndpoint, DeviceAccessTokenEndpointError, DeviceAuthorizationEndpoint,
    DeviceAuthorizationEndpointError,
};

pub struct Flow<C>
where
    C: RetryableClient,
{
    client: C,
}
impl<C> Flow<C>
where
    C: RetryableClient,
{
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

impl<'a, C> Flow<C>
where
    C: RetryableClient + Send + Sync,
{
    pub async fn start<P, UI>(
        &self,
        provider: &'a P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        user_interaction: UI,
    ) -> Result<DATRES_SuccessfulBody<<P as Provider>::Scope>, FlowError>
    where
        P: ProviderExtDeviceAuthorizationGrant + Send + Sync,
        <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
        <P as Provider>::Scope: Serialize + DeserializeOwned + Send + Sync,
        UI: FnOnce(UserCode, VerificationUri, Option<VerificationUriComplete>),
    {
        // Step 1
        let device_authorization_endpoint = DeviceAuthorizationEndpoint::new(provider, scopes);

        let device_authorization_ret = self
            .client
            .respond_endpoint(&device_authorization_endpoint)
            .await
            .map_err(|err| FlowError::DeviceAuthorizationEndpointRespondFailed(err.to_string()))?;

        let device_authorization_successful_body =
            device_authorization_ret.map_err(FlowError::DeviceAuthorizationFailed)?;

        // Step 2
        user_interaction(
            device_authorization_successful_body.user_code.to_owned(),
            device_authorization_successful_body
                .verification_uri
                .to_owned(),
            device_authorization_successful_body
                .verification_uri_complete
                .to_owned(),
        );

        // Step 3
        let device_access_token_endpoint = DeviceAccessTokenEndpoint::new(
            provider,
            device_authorization_successful_body.device_code.to_owned(),
            device_authorization_successful_body.interval(),
        );

        let device_access_token_ret = self
            .client
            .respond_endpoint_until_done(&device_access_token_endpoint)
            .await
            .map_err(|err| FlowError::DeviceAccessTokenEndpointRespondFailed(err.to_string()))?;

        let device_access_token_successful_body =
            device_access_token_ret.map_err(FlowError::DeviceAccessTokenFailed)?;

        Ok(device_access_token_successful_body)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FlowError {
    #[error("DeviceAuthorizationEndpointError {0}")]
    DeviceAuthorizationEndpointError(DeviceAuthorizationEndpointError),
    #[error("DeviceAuthorizationEndpointRespondFailed {0}")]
    DeviceAuthorizationEndpointRespondFailed(String),
    #[error("DeviceAuthorizationFailed {0:?}")]
    DeviceAuthorizationFailed(DARES_ErrorBody),
    //
    #[error("DeviceAccessTokenEndpointError {0}")]
    DeviceAccessTokenEndpointError(DeviceAccessTokenEndpointError),
    #[error("DeviceAccessTokenEndpointRespondFailed {0}")]
    DeviceAccessTokenEndpointRespondFailed(String),
    #[error("DeviceAccessTokenFailed {0:?}")]
    DeviceAccessTokenFailed(DATRES_ErrorBody),
}
