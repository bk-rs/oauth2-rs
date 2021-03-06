use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::GoogleScope;

use super::GoogleUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct GoogleExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for GoogleExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            GoogleUserInfoEndpoint::new(
                &access_token.access_token,
                access_token.scope.as_ref().map(|x| {
                    ScopeParameter::<String>::from(x)
                        .0
                        .contains(&GoogleScope::Openid.to_string())
                }) == Some(true),
            ),
        )))
    }
}
