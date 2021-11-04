use super::{AccessTokenObtainFrom, EndpointExecuteError};
use crate::re_exports::{async_trait, AccessTokenResponseSuccessfulBody, Client};

#[async_trait]
pub trait RevokeAccessTokenEndpoint {
    fn can_execute(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<String>,
    ) -> bool;

    async fn execute<C>(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<String>,
        client: &C,
    ) -> Result<(), EndpointExecuteError>
    where
        C: Client + Send + Sync,
        Self: Sized;
}
