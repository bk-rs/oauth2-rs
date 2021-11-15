use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::Scope,
};

use super::BitbucketUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct BitbucketExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for BitbucketExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            BitbucketUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
