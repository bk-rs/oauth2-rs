use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, EndpointBuilder, GrantInfo, UserInfo,
        UserInfoObtainOutput,
    },
    re_exports::Scope,
};

//
#[derive(Debug, Clone)]
pub struct DigitaloceanEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for DigitaloceanEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_provider: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        let info = access_token
            .extensions()
            .ok_or("extensions missing")?
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

        Ok(UserInfoObtainOutput::Static(UserInfo {
            uid,
            name: Some(name),
            email: Some(email),
            raw: info.to_owned(),
        }))
    }
}
