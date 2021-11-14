use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfoObtainOutput,
    },
    re_exports::Scope,
};

use super::TwitchUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct TwitchEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for TwitchEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        access_token_provider: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        let client_id = match access_token_provider {
            GrantInfo::AuthorizationCodeGrant {
                provider,
                authorization_request_scopes: _,
            } => provider.client_id(),
            GrantInfo::DeviceAuthorizationGrant {
                provider,
                authorization_request_scopes: _,
            } => provider.client_id(),
        };
        let client_id = client_id.ok_or("missing client_id")?;

        Ok(UserInfoObtainOutput::Respond(Box::new(
            TwitchUserInfoEndpoint::new(&access_token.access_token, client_id),
        )))
    }
}
