use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo, UserInfo,
    },
    re_exports::Scope,
};

//
#[derive(Debug, Clone)]
pub struct DigitaloceanExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for DigitaloceanExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        let info = access_token
            .extra()
            .ok_or("extra missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .get("info")
            .ok_or("info missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .as_object()
            .ok_or("info mismatch")
            .map_err(BuilderObtainUserInfoError::Unreachable)?;

        let uid = info
            .get("uuid")
            .ok_or("uuid missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .as_str()
            .ok_or("uuid mismatch")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .to_owned();

        let name = info
            .get("name")
            .ok_or("name missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .as_str()
            .ok_or("name mismatch")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .to_owned();

        let email = info
            .get("email")
            .ok_or("email missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .as_str()
            .ok_or("email mismatch")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .to_owned();

        Ok(BuilderObtainUserInfoOutput::Static(UserInfo {
            uid,
            name: Some(name),
            email: Some(email),
            raw: info.to_owned(),
        }))
    }
}
