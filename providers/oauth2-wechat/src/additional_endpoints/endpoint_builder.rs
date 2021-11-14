use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenProvider, AccessTokenResponseSuccessfulBody, EndpointBuilder, UserInfo,
        UserInfoObtainOutput,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::{authorization_code_grant::KEY_OPENID, WechatScope};

use super::WechatUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct WechatEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for WechatEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        access_token_provider: AccessTokenProvider<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>> {
        if let Some(scope) = &access_token.scope {
            let scope = ScopeParameter::<String>::from(scope).0;
            if scope.contains(&WechatScope::SnsapiLogin.to_string()) {
                let openid = access_token
                    .extensions()
                    .ok_or("extensions missing")?
                    .get(KEY_OPENID)
                    .ok_or("openid missing")?
                    .as_str()
                    .ok_or("openid mismatch")?
                    .to_owned();

                return Ok(UserInfoObtainOutput::Respond(Box::new(
                    WechatUserInfoEndpoint::new(&access_token.access_token, openid),
                )));
            }
        }

        match access_token_provider {
            AccessTokenProvider::AuthorizationCodeGrant(_) => {
                let uid = access_token
                    .extensions()
                    .ok_or("extensions missing")?
                    .get(KEY_OPENID)
                    .ok_or("openid missing")?
                    .as_str()
                    .ok_or("openid mismatch")?
                    .to_owned();

                return Ok(UserInfoObtainOutput::Static(UserInfo {
                    uid,
                    name: None,
                    email: None,
                    raw: Default::default(),
                }));
            }
            AccessTokenProvider::DeviceAuthorizationGrant(_) => {}
        }

        Ok(UserInfoObtainOutput::None)
    }
}
