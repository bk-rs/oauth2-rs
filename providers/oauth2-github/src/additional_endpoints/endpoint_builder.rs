use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointBuilder,
        UserInfoEndpointBuildOutput,
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
    fn user_info_endpoint_build(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> UserInfoEndpointBuildOutput {
        UserInfoEndpointBuildOutput::Respond(Box::new(GithubUserInfoEndpoint::new(
            &access_token.access_token,
        )))
    }
}
