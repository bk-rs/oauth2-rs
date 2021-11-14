use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfoObtainOutput,
    },
    re_exports::Scope,
};

use super::LinkedinUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct LinkedinEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for LinkedinEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_provider: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(UserInfoObtainOutput::Respond(Box::new(
            LinkedinUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
