use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointBuilder,
        UserInfoObtainRet,
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
    fn user_info_obtain(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> UserInfoObtainRet {
        UserInfoObtainRet::Respond(Box::new(GoogleUserInfoEndpoint::new(
            &access_token.access_token,
        )))
    }
}
