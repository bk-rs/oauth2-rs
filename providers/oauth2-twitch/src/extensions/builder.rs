use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::Scope,
};

use super::TwitchUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct TwitchExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for TwitchExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        let client_id = match grant_info {
            GrantInfo::AuthorizationCodeGrant(info) => info.provider.client_id(),
            GrantInfo::DeviceAuthorizationGrant(info) => info.provider.client_id(),
        };
        let client_id = client_id.ok_or("missing client_id")?;

        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            TwitchUserInfoEndpoint::new(&access_token.access_token, client_id),
        )))
    }
}
