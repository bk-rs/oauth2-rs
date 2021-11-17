use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::Scope,
};

use super::InstagramUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct InstagramExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for InstagramExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        let ig_user_id = access_token
            .extra()
            .map(|x| x.get("user_id").cloned())
            .ok_or("user_id missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .ok_or("user_id missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .as_u64()
            .ok_or("user_id mismatch")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .to_owned();

        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            InstagramUserInfoEndpoint::new(&access_token.access_token, ig_user_id),
        )))
    }
}
