use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, ExtensionsBuilder,
        GrantInfo,
    },
    re_exports::Scope,
};

use super::FacebookUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct FacebookExtensionsBuilder;

impl<SCOPE> ExtensionsBuilder<SCOPE> for FacebookExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            FacebookUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
