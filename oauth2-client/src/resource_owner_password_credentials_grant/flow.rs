use std::error;

use http_api_client::{Client, ClientRespondEndpointError};
use oauth2_core::{
    resource_owner_password_credentials_grant::access_token_response::{
        ErrorBody as AT_RES_ErrorBody, SuccessfulBody as AT_RES_SuccessfulBody,
    },
    serde::{de::DeserializeOwned, Serialize},
    types::Scope,
};

use crate::ProviderExtResourceOwnerPasswordCredentialsGrant;

use super::{AccessTokenEndpoint, AccessTokenEndpointError};

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

impl<C> Flow<C>
where
    C: Client + Send + Sync,
{
    pub async fn execute<SCOPE>(
        &self,
        provider: &(dyn ProviderExtResourceOwnerPasswordCredentialsGrant<Scope = SCOPE>
              + Send
              + Sync),
        scopes: impl Into<Option<Vec<SCOPE>>>,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<AT_RES_SuccessfulBody<SCOPE>, FlowExecuteError>
    where
        SCOPE: Scope + Serialize + DeserializeOwned + Send + Sync,
    {
        // Step 1
        let scopes = scopes.into().or_else(|| provider.scopes_default());

        let access_token_endpoint = AccessTokenEndpoint::new(provider, scopes, username, password);

        let access_token_ret = self
            .client_with_token
            .respond_endpoint(&access_token_endpoint)
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    FlowExecuteError::AccessTokenEndpointRespondFailed(Box::new(err))
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => {
                    FlowExecuteError::AccessTokenEndpointError(err)
                }
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => {
                    FlowExecuteError::AccessTokenEndpointError(err)
                }
            })?;

        let access_token_successful_body =
            access_token_ret.map_err(FlowExecuteError::AccessTokenFailed)?;

        Ok(access_token_successful_body)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FlowExecuteError {
    #[error("AccessTokenEndpointRespondFailed {0}")]
    AccessTokenEndpointRespondFailed(Box<dyn error::Error + Send + Sync>),
    #[error("AccessTokenEndpointError {0}")]
    AccessTokenEndpointError(AccessTokenEndpointError),
    #[error("AccessTokenFailed {0:?}")]
    AccessTokenFailed(AT_RES_ErrorBody),
}
