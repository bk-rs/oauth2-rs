use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, ExtensionsBuilder,
        GrantInfo,
    },
    re_exports::Scope,
};

use super::LinkedinUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct LinkedinExtensionsBuilder;

impl<SCOPE> ExtensionsBuilder<SCOPE> for LinkedinExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            LinkedinUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
