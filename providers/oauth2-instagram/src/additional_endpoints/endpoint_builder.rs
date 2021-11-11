use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenProvider, AccessTokenResponseSuccessfulBody, EndpointBuilder,
        UserInfoObtainOutput,
    },
    re_exports::Scope,
};

use super::InstagramUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct InstagramEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for InstagramEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_provider: AccessTokenProvider<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        let ig_user_id = access_token
            .extensions()
            .map(|x| x.get("user_id").cloned())
            .ok_or_else(|| "Missing user_id")?
            .ok_or_else(|| "Missing user_id")?
            .as_u64()
            .ok_or_else(|| "Mismatch user_id")?
            .to_owned();

        Ok(UserInfoObtainOutput::Respond(Box::new(
            InstagramUserInfoEndpoint::new(&access_token.access_token, ig_user_id),
        )))
    }
}
