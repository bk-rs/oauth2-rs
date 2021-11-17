use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::Scope,
};

use super::LinkedinUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct LinkedinExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for LinkedinExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            LinkedinUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
