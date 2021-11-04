use std::error;

use super::{AccessTokenObtainFrom, UserInfo};
use crate::re_exports::{async_trait, AccessTokenResponseSuccessfulBody, Client};

#[async_trait]
pub trait UserInfoEndpoint {
    type Output: TryInto<UserInfo>;
    type Error: error::Error + 'static;

    fn can_execute(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<String>,
    ) -> bool;

    async fn execute<C1, C2>(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<String>,
        client: &C1,
        _: &C2,
    ) -> Result<Self::Output, Self::Error>
    where
        C1: Client + Send + Sync,
        C2: Client + Send + Sync;
}
