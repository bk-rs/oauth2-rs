use http_api_client::{
    Client, ClientRespondEndpointError, RetryableClient,
    RetryableClientRespondEndpointUntilDoneError,
};
use oauth2_core::{
    device_authorization_grant::{
        device_access_token_response::{
            ErrorBody as DAT_RES_ErrorBody, SuccessfulBody as DAT_RES_SuccessfulBody,
        },
        device_authorization_response::{
            ErrorBody as DA_RES_ErrorBody, UserCode, VerificationUri, VerificationUriComplete,
        },
    },
    serde::{de::DeserializeOwned, Serialize},
};

use crate::{Provider, ProviderExtDeviceAuthorizationGrant};

use super::{
    DeviceAccessTokenEndpoint, DeviceAccessTokenEndpointError, DeviceAuthorizationEndpoint,
    DeviceAuthorizationEndpointError,
};

//
//
//
#[derive(Debug, Clone)]
pub struct Flow<C1, C2>
where
    C1: Client,
    C2: RetryableClient,
{
    client_with_auth: C1,
    client_with_token: C2,
}
impl<C1, C2> Flow<C1, C2>
where
    C1: Client,
    C2: RetryableClient,
{
    pub fn new(client_with_auth: C1, client_with_token: C2) -> Self {
        Self {
            client_with_auth,
            client_with_token,
        }
    }
}

impl<'a, C1, C2> Flow<C1, C2>
where
    C1: Client + Send + Sync,
    C2: RetryableClient + Send + Sync,
{
    pub async fn execute<P, UI>(
        &self,
        provider: &'a P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        user_interaction: UI,
    ) -> Result<DAT_RES_SuccessfulBody<<P as Provider>::Scope>, FlowExecuteError>
    where
        P: ProviderExtDeviceAuthorizationGrant,

        <P as Provider>::Scope: Serialize + DeserializeOwned + Send + Sync,
        UI: FnOnce(UserCode, VerificationUri, Option<VerificationUriComplete>),
    {
        // Step 1
        let scopes = scopes.into().or(provider.scopes_default());

        let device_authorization_endpoint = DeviceAuthorizationEndpoint::new(provider, scopes);

        let device_authorization_ret = self
            .client_with_auth
            .respond_endpoint(&device_authorization_endpoint)
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    FlowExecuteError::DeviceAuthorizationEndpointRespondFailed(err.to_string())
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => {
                    FlowExecuteError::DeviceAuthorizationEndpointError(err)
                }
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => {
                    FlowExecuteError::DeviceAuthorizationEndpointError(err)
                }
            })?;

        let device_authorization_successful_body =
            device_authorization_ret.map_err(FlowExecuteError::DeviceAuthorizationFailed)?;

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
            .client_with_token
            .respond_endpoint_until_done(&device_access_token_endpoint)
            .await
            .map_err(|err| match err {
                RetryableClientRespondEndpointUntilDoneError::RespondFailed(err) => {
                    FlowExecuteError::DeviceAccessTokenEndpointRespondFailed(err.to_string())
                }
                RetryableClientRespondEndpointUntilDoneError::EndpointRenderRequestFailed(err) => {
                    FlowExecuteError::DeviceAccessTokenEndpointError(err)
                }
                RetryableClientRespondEndpointUntilDoneError::EndpointParseResponseFailed(err) => {
                    FlowExecuteError::DeviceAccessTokenEndpointError(err)
                }
                RetryableClientRespondEndpointUntilDoneError::ReachedMaxRetries => {
                    FlowExecuteError::DeviceAccessTokenEndpointRespondFailed(
                        "ReachedMaxRetries".to_owned(),
                    )
                }
            })?;

        let device_access_token_successful_body =
            device_access_token_ret.map_err(FlowExecuteError::DeviceAccessTokenFailed)?;

        Ok(device_access_token_successful_body)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FlowExecuteError {
    #[error("DeviceAuthorizationEndpointRespondFailed {0}")]
    DeviceAuthorizationEndpointRespondFailed(String),
    #[error("DeviceAuthorizationEndpointError {0}")]
    DeviceAuthorizationEndpointError(DeviceAuthorizationEndpointError),
    #[error("DeviceAuthorizationFailed {0:?}")]
    DeviceAuthorizationFailed(DA_RES_ErrorBody),
    //
    #[error("DeviceAccessTokenEndpointRespondFailed {0}")]
    DeviceAccessTokenEndpointRespondFailed(String),
    #[error("DeviceAccessTokenEndpointError {0}")]
    DeviceAccessTokenEndpointError(DeviceAccessTokenEndpointError),
    #[error("DeviceAccessTokenFailed {0:?}")]
    DeviceAccessTokenFailed(DAT_RES_ErrorBody),
}
