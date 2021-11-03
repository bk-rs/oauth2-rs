use std::{error, fmt, str};

pub use http_api_client::{self, async_trait, Client, ClientRespondEndpointError};
pub use http_api_endpoint::{self, Endpoint};
pub use oauth2_core::access_token_response::GeneralSuccessfulBody as AccessTokenResponseSuccessfulBody;

use crate::Provider;

#[async_trait]
pub trait ProviderExtUserInfo: Provider {
    type Output: UserInfo;
    type Error: error::Error + 'static;

    async fn fetch_user_info<C2, C3>(
        &self,
        token_source: AccessTokenResponseSuccessfulBodySource,
        token: &AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
        client: &C2,
        another_client: &C3,
    ) -> Result<Self::Output, Self::Error>
    where
        <Self::Scope as str::FromStr>::Err: fmt::Display,
        Self::Scope: Send + Sync,
        C2: Client + Send + Sync,
        C3: Client + Send + Sync;
}

pub enum AccessTokenResponseSuccessfulBodySource {
    AuthorizationCodeGrant,
    DeviceAuthorizationGrant,
}

pub trait UserInfo {
    fn uid(&self) -> String;
    fn name(&self) -> Option<String>;
    fn email(&self) -> Option<String>;
}
