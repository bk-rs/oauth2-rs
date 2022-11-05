use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::{serde_json, Scope},
};

use super::TiktokUserInfoEndpoint;
use crate::TiktokScope;

//
#[derive(Debug, Clone)]
pub struct TiktokExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for TiktokExtensionsBuilder
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
                .contains(TiktokScope::UserInfoBasic.to_string().as_str())
            {
                return Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
                    TiktokUserInfoEndpoint::new(&access_token.access_token),
                )));
            }
        }

        Ok(BuilderObtainUserInfoOutput::None)
    }
}
