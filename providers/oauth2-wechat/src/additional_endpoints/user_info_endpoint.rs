///! Ref https://github.com/NeverMin/omniauth-wechat-oauth2/blob/master/lib/omniauth/strategies/wechat.rb
use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointOutputObtainFrom,
        EndpointParseResponseError, EndpointRenderRequestError, UserInfo, UserInfoEndpoint,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::{Body, Request, Response, Scope},
};

use crate::WeChatScope;

//
#[derive(Debug, Clone)]
pub struct WeChatUserInfoEndpoint;

impl<SCOPE> UserInfoEndpoint<SCOPE> for WeChatUserInfoEndpoint
where
    SCOPE: Scope,
{
    fn obtain_from(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> EndpointOutputObtainFrom {
        if let Some(scope) = &access_token.scope {
            let scope = ScopeParameter::<String>::from(scope).0;
            if scope.contains(&WeChatScope::SnsapiLogin.to_string()) {
                return EndpointOutputObtainFrom::Respond;
            }
        }

        match access_token_obtain_from {
            AccessTokenObtainFrom::AuthorizationCodeGrant => {
                return EndpointOutputObtainFrom::Build
            }
            AccessTokenObtainFrom::DeviceAuthorizationGrant => {}
        }

        EndpointOutputObtainFrom::None
    }

    fn build(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfo, Box<dyn error::Error + 'static>> {
        match access_token_obtain_from {
            AccessTokenObtainFrom::AuthorizationCodeGrant => {
                let uid = access_token
                    .extensions()
                    .ok_or_else(|| "extensions missing")?
                    .get("openid")
                    .ok_or_else(|| "openid missing")?
                    .as_str()
                    .ok_or_else(|| "openid mismatch")?
                    .to_owned();

                Ok(UserInfo {
                    uid,
                    name: None,
                    email: None,
                    raw: Default::default(),
                })
            }
            AccessTokenObtainFrom::DeviceAuthorizationGrant => {
                unreachable!()
            }
        }
    }

    fn render_request(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<Request<Body>, EndpointRenderRequestError> {
        todo!()
    }

    fn parse_response(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
        _response: Response<Body>,
    ) -> Result<UserInfo, EndpointParseResponseError> {
        todo!()
    }
}
