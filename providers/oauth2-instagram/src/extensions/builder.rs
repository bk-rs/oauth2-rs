use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, ExtensionsBuilder,
        GrantInfo,
    },
    re_exports::Scope,
};

use super::InstagramUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct InstagramExtensionsBuilder;

impl<SCOPE> ExtensionsBuilder<SCOPE> for InstagramExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        let ig_user_id = access_token
            .extra()
            .map(|x| x.get("user_id").cloned())
            .ok_or("Missing user_id")?
            .ok_or("Missing user_id")?
            .as_u64()
            .ok_or("Mismatch user_id")?
            .to_owned();

        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            InstagramUserInfoEndpoint::new(&access_token.access_token, ig_user_id),
        )))
    }
}
