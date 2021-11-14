use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, EndpointBuilder, GrantInfo,
    },
    re_exports::Scope,
};

use super::GithubUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct GithubEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for GithubEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            GithubUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
