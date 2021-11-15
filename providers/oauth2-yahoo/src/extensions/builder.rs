use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::Scope,
};

use super::YahooUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct YahooExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for YahooExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        if let Some(_id_token) = &access_token.id_token {
            // TODO
        }

        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            YahooUserInfoEndpoint::new(&access_token.access_token),
        )))
    }
}
