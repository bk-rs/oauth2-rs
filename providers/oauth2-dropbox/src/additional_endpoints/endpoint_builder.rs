use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfo,
        UserInfoObtainOutput,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::DropboxScope;

use super::DropboxUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct DropboxEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for DropboxEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        let extensions = access_token.extensions().ok_or("extensions missing")?;

        let uid = extensions
            .get("uid")
            .ok_or("uid missing")?
            .as_str()
            .ok_or("uid mismatch")?
            .to_owned();

        let scopes = access_token
            .scope
            .to_owned()
            .map(|x| ScopeParameter::<String>::from(&x).0)
            .unwrap_or_default();

        if scopes.contains(&DropboxScope::SharingRead.to_string()) {
            let account_id = extensions
                .get("account_id")
                .ok_or("account_id missing")?
                .as_str()
                .ok_or("account_id mismatch")?
                .to_owned();

            return Ok(UserInfoObtainOutput::Respond(Box::new(
                DropboxUserInfoEndpoint::new(&access_token.access_token, account_id, uid),
            )));
        }

        Ok(UserInfoObtainOutput::Static(UserInfo {
            uid,
            name: None,
            email: None,
            raw: extensions.to_owned(),
        }))
    }
}
