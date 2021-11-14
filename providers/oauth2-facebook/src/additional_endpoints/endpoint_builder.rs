use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfoObtainOutput,
    },
    re_exports::Scope,
};

use super::FacebookUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct FacebookEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for FacebookEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_provider: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(UserInfoObtainOutput::Respond(Box::new(
            FacebookUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
