use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, Builder,
        GrantInfo,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::GitlabScope;

use super::GitlabUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct GitlabExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for GitlabExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        let scopes = access_token
            .scope
            .to_owned()
            .map(|x| ScopeParameter::<String>::from(&x).0)
            .unwrap_or_default();

        if scopes.contains(&GitlabScope::ReadUser.to_string()) {
            return Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
                GitlabUserInfoEndpoint::new(&access_token.access_token),
            )));
        }

        Ok(BuilderObtainUserInfoOutput::None)
    }
}
