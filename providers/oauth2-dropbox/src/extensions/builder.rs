use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo, UserInfo,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::DropboxScope;

use super::DropboxUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct DropboxExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for DropboxExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        let extra = access_token
            .extra()
            .ok_or("extra missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?;

        let uid = extra
            .get("uid")
            .ok_or("uid missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .as_str()
            .ok_or("uid mismatch")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .to_owned();

        let scopes = access_token
            .scope
            .to_owned()
            .map(|x| ScopeParameter::<String>::from(&x).0)
            .unwrap_or_default();

        if scopes.contains(&DropboxScope::SharingRead.to_string()) {
            let account_id = extra
                .get("account_id")
                .ok_or("account_id missing")
                .map_err(BuilderObtainUserInfoError::Unreachable)?
                .as_str()
                .ok_or("account_id mismatch")
                .map_err(BuilderObtainUserInfoError::Unreachable)?
                .to_owned();

            return Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
                DropboxUserInfoEndpoint::new(&access_token.access_token, account_id, uid),
            )));
        }

        Ok(BuilderObtainUserInfoOutput::Static(UserInfo {
            uid,
            name: None,
            email: None,
            raw: extra.to_owned(),
        }))
    }
}
