use std::error;

use oauth2_client::{
    extensions::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_pan_uinfo_endpoint::{PanUinfoEndpoint, PanUinfoEndpointError, Uinfo};

//
#[derive(Debug, Clone)]
pub struct BaiduUserInfoEndpoint {
    inner: PanUinfoEndpoint,
}
impl BaiduUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            inner: PanUinfoEndpoint::new(access_token),
        }
    }
}

impl Endpoint for BaiduUserInfoEndpoint {
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
        UserInfoWrapper::try_from(self.inner.parse_response(response)?)
            .map(|x| x.0)
            .map_err(EndpointParseResponseError::ToOutputFailed)
    }
}

//
impl From<PanUinfoEndpointError> for EndpointRenderRequestError {
    fn from(err: PanUinfoEndpointError) -> Self {
        match err {
            PanUinfoEndpointError::UrlParseFailed(err) => Self::Other(Box::new(err)),
            PanUinfoEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            PanUinfoEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<PanUinfoEndpointError> for EndpointParseResponseError {
    fn from(err: PanUinfoEndpointError) -> Self {
        match err {
            PanUinfoEndpointError::UrlParseFailed(err) => Self::Other(Box::new(err)),
            PanUinfoEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            PanUinfoEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
        }
    }
}

//
struct UserInfoWrapper(UserInfo);
impl TryFrom<Uinfo> for UserInfoWrapper {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(user_info: Uinfo) -> Result<Self, Self::Error> {
        Ok(Self(UserInfo {
            uid: user_info.uk.to_string(),
            name: Some(user_info.baidu_name.to_owned()),
            email: None,
            raw: serde_json::to_value(user_info)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        }))
    }
}
