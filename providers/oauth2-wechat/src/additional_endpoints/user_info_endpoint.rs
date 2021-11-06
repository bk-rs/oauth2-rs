use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointOutputObtainFrom,
        EndpointParseResponseError, EndpointRenderRequestError, UserInfo, UserInfoEndpoint,
    },
    oauth2_core::types::ScopeParameter,
    re_exports::{serde_json, Body, Endpoint as _, Map, Request, Response, Scope, Value},
};

use crate::{authorization_code_grant::KEY_OPENID, WeChatScope};

use super::internal_sns_userinfo_endpoint::{
    SnsUserinfo, SnsUserinfoEndpoint, SnsUserinfoEndpointError,
};

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
                    .get(KEY_OPENID)
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
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<Request<Body>, EndpointRenderRequestError> {
        let endpoint = SnsUserinfoEndpoint::new(
            &access_token.access_token,
            &access_token
                .extensions()
                .cloned()
                .unwrap_or_else(|| Map::new())
                .get(KEY_OPENID)
                .cloned()
                .unwrap_or_else(|| Value::String("".to_owned()))
                .as_str()
                .unwrap_or_default()
                .to_owned(),
        );

        endpoint.render_request().map_err(Into::into)
    }

    fn parse_response(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
        response: Response<Body>,
    ) -> Result<UserInfo, EndpointParseResponseError> {
        let endpoint = SnsUserinfoEndpoint::new("", "");

        let user = endpoint.parse_response(response)?;

        Ok(UserInfo::try_from(user).map_err(EndpointParseResponseError::ToOutputFailed)?)
    }
}

//
impl From<SnsUserinfoEndpointError> for EndpointRenderRequestError {
    fn from(err: SnsUserinfoEndpointError) -> Self {
        match err {
            SnsUserinfoEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            SnsUserinfoEndpointError::DeResponseBodyFailed(err) => Self::Other(err.to_string()),
        }
    }
}
impl From<SnsUserinfoEndpointError> for EndpointParseResponseError {
    fn from(err: SnsUserinfoEndpointError) -> Self {
        match err {
            SnsUserinfoEndpointError::MakeRequestFailed(err) => Self::Other(err.to_string()),
            SnsUserinfoEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
        }
    }
}

//
impl TryFrom<SnsUserinfo> for UserInfo {
    type Error = String;

    fn try_from(sns_userinfo: SnsUserinfo) -> Result<Self, Self::Error> {
        Ok(Self {
            uid: sns_userinfo.openid.to_owned(),
            name: sns_userinfo.nickname.to_owned(),
            email: None,
            raw: serde_json::to_value(sns_userinfo)
                .map(|x| x.as_object().cloned())
                .map_err(|err| err.to_string())?
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
