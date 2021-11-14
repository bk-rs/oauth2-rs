use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, EndpointBuilder, GrantInfo,
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
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            LinkedinUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
