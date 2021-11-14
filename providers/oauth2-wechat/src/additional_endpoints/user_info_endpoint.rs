use std::error;

use oauth2_client::{
    additional_endpoints::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_sns_userinfo_endpoint::{
    SnsUserinfo, SnsUserinfoEndpoint, SnsUserinfoEndpointError,
};

//
#[derive(Debug, Clone)]
pub struct WechatUserInfoEndpoint {
    inner: SnsUserinfoEndpoint,
}
impl WechatUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>, openid: impl AsRef<str>) -> Self {
        Self {
            inner: SnsUserinfoEndpoint::new(access_token, openid),
        }
    }
}

impl Endpoint for WechatUserInfoEndpoint {
    type RenderRequestError = EndpointRenderRequestError;

    type ParseResponseOutput = UserInfo;
    type ParseResponseError = EndpointParseResponseError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        self.inner.render_request().map_err(Into::into)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        UserInfo::try_from(self.inner.parse_response(response)?)
            .map_err(EndpointParseResponseError::ToOutputFailed)
    }
}

//
impl From<SnsUserinfoEndpointError> for EndpointRenderRequestError {
    fn from(err: SnsUserinfoEndpointError) -> Self {
        match err {
            SnsUserinfoEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            SnsUserinfoEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<SnsUserinfoEndpointError> for EndpointParseResponseError {
    fn from(err: SnsUserinfoEndpointError) -> Self {
        match err {
            SnsUserinfoEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            SnsUserinfoEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
        }
    }
}

//
impl TryFrom<SnsUserinfo> for UserInfo {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(sns_userinfo: SnsUserinfo) -> Result<Self, Self::Error> {
        Ok(Self {
            uid: sns_userinfo.openid.to_owned(),
            name: sns_userinfo.nickname.to_owned(),
            email: None,
            raw: serde_json::to_value(sns_userinfo)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
