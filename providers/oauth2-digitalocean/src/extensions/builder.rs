use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoOutput, GrantInfo,
        UserInfo,
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
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        let info = access_token
            .extra()
            .ok_or("extra missing")?
            .get("info")
            .ok_or("info missing")?
            .as_object()
            .ok_or("openid mismatch")?;

        let uid = info
            .get("uuid")
            .ok_or("uuid missing")?
            .as_str()
            .ok_or("uuid mismatch")?
            .to_owned();

        let name = info
            .get("name")
            .ok_or("name missing")?
            .as_str()
            .ok_or("name mismatch")?
            .to_owned();

        let email = info
            .get("email")
            .ok_or("email missing")?
            .as_str()
            .ok_or("email mismatch")?
            .to_owned();

        Ok(BuilderObtainUserInfoOutput::Static(UserInfo {
            uid,
            name: Some(name),
            email: Some(email),
            raw: info.to_owned(),
        }))
    }
}
