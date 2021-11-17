use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::Scope,
};

use super::MicrosoftUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct MicrosoftExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for MicrosoftExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            MicrosoftUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
