use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, EndpointBuilder, GrantInfo,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::GoogleScope;

use super::GoogleUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct GoogleEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for GoogleEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
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
