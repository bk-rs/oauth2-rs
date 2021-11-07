use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointBuilder,
        UserInfoEndpointBuildOutput,
    },
    re_exports::Scope,
};

use super::GoogleUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct GoogleEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for GoogleEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_endpoint_build(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> UserInfoEndpointBuildOutput {
        UserInfoEndpointBuildOutput::Respond(Box::new(GoogleUserInfoEndpoint::new(
            &access_token.access_token,
        )))
    }
}
