use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfoObtainOutput,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::GitlabScope;

use super::GitlabUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct GitlabEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for GitlabEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_provider: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        let scopes = access_token
            .scope
            .to_owned()
            .map(|x| ScopeParameter::<String>::from(&x).0)
            .unwrap_or_default();

        if scopes.contains(&GitlabScope::ReadUser.to_string()) {
            return Ok(UserInfoObtainOutput::Respond(Box::new(
                GitlabUserInfoEndpoint::new(&access_token.access_token),
            )));
        }

        Ok(UserInfoObtainOutput::None)
    }
}
