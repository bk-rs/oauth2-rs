use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenProvider, AccessTokenResponseSuccessfulBody, EndpointBuilder,
        UserInfoObtainOutput,
    },
    re_exports::Scope,
};

use super::AmazonUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct AmazonEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for AmazonEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_provider: AccessTokenProvider<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(UserInfoObtainOutput::Respond(Box::new(
            AmazonUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
