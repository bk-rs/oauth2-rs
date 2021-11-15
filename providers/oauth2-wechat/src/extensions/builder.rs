use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, Builder,
        GrantInfo, UserInfo,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::Scope,
};

use crate::{authorization_code_grant::KEY_OPENID, WechatScope};

use super::WechatUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct WechatExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for WechatExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        let has_snsapi_login_scope = access_token.scope.as_ref().map(|x| {
            ScopeParameter::<String>::from(x)
                .0
                .contains(&WechatScope::SnsapiLogin.to_string())
        }) == Some(true);

        if has_snsapi_login_scope {
            let openid = access_token
                .extra()
                .ok_or("extra missing")?
                .get(KEY_OPENID)
                .ok_or("openid missing")?
                .as_str()
                .ok_or("openid mismatch")?
                .to_owned();

            return Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
                WechatUserInfoEndpoint::new(&access_token.access_token, openid),
            )));
        }

        match grant_info {
            GrantInfo::AuthorizationCodeGrant(_) => {
                let uid = access_token
                    .extra()
                    .ok_or("extra missing")?
                    .get(KEY_OPENID)
                    .ok_or("openid missing")?
                    .as_str()
                    .ok_or("openid mismatch")?
                    .to_owned();

                return Ok(BuilderObtainUserInfoOutput::Static(UserInfo {
                    uid,
                    name: None,
                    email: None,
                    raw: Default::default(),
                }));
            }
            GrantInfo::DeviceAuthorizationGrant(_) => {
                // unknown
            }
        }

        Ok(BuilderObtainUserInfoOutput::None)
    }
}
