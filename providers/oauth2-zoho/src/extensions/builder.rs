use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::{serde_json, Scope},
};

use super::ZohoUserInfoEndpoint;
use crate::ZohoScope;

//
#[derive(Debug, Clone)]
pub struct ZohoExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for ZohoExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        if let Some(scope) = &access_token.scope {
            let scope = serde_json::from_str::<ScopeParameter<ZohoScope>>(
                serde_json::to_string(&scope).unwrap_or_default().as_str(),
            )
            .unwrap_or(ScopeParameter::<ZohoScope>(vec![]));

            // AaaServer.profile.READ && !profile
            if scope.0.contains(&ZohoScope::AaaServerProfileRead)
                && !scope.0.contains(&ZohoScope::Profile)
            {
                return Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
                    ZohoUserInfoEndpoint::new(&access_token.access_token),
                )));
            }
        }

        Ok(BuilderObtainUserInfoOutput::None)
    }
}
