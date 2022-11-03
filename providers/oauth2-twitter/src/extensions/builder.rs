use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::{serde_json, Scope},
};

use super::TwitterUserInfoEndpoint;
use crate::TwitterScope;

//
#[derive(Debug, Clone)]
pub struct TwitterExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for TwitterExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        if let Some(scope) = &access_token.scope {
            if serde_json::to_string(&scope)
                .unwrap_or_default()
                .contains(TwitterScope::UsersRead.to_string().as_str())
                && serde_json::to_string(&scope)
                    .unwrap_or_default()
                    .contains(TwitterScope::TweetRead.to_string().as_str())
            {
                return Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
                    TwitterUserInfoEndpoint::new(&access_token.access_token),
                )));
            }
        }

        Ok(BuilderObtainUserInfoOutput::None)
    }
}
