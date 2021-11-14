use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenProvider, AccessTokenResponseSuccessfulBody, EndpointBuilder, UserInfo,
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
        _access_token_provider: AccessTokenProvider<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        let info = access_token
            .extensions()
            .ok_or_else(|| "extensions missing")?
            .get("info")
            .ok_or_else(|| "info missing")?
            .as_object()
            .ok_or_else(|| "openid mismatch")?;

        let uid = info
            .get("uuid")
            .ok_or_else(|| "uuid missing")?
            .as_str()
            .ok_or_else(|| "uuid mismatch")?
            .to_owned();

        let name = info
            .get("name")
            .ok_or_else(|| "name missing")?
            .as_str()
            .ok_or_else(|| "name mismatch")?
            .to_owned();

        let email = info
            .get("email")
            .ok_or_else(|| "email missing")?
            .as_str()
            .ok_or_else(|| "email mismatch")?
            .to_owned();

        return Ok(UserInfoObtainOutput::Static(UserInfo {
            uid,
            name: Some(name),
            email: Some(email),
            raw: info.to_owned(),
        }));
    }
}
