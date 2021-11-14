use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, EndpointBuilder, GrantInfo,
    },
    re_exports::Scope,
};

use super::BitbucketUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct BitbucketEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for BitbucketEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            BitbucketUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
