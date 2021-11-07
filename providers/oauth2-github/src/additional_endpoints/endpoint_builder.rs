use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointBuilder,
        UserInfoObtainOutput,
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
        _access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + 'static>> {
        Ok(UserInfoObtainOutput::Respond(Box::new(
            GithubUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
