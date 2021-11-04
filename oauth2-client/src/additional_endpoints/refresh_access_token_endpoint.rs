use std::error;

use super::{async_trait, AccessTokenObtainFrom, Client};
use crate::re_exports::AccessTokenResponseSuccessfulBody;

#[async_trait]
pub trait RefreshAccessTokenEndpoint {
    type Output: TryInto<AccessTokenResponseSuccessfulBody<String>>;
    type Error: error::Error + 'static;

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
    ) -> Result<Self::Output, Self::Error>
    where
        C: Client + Send + Sync;
}
