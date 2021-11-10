use std::error;

use oauth2_client::{
    additional_endpoints::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_oauth2_v3_user_info_endpoint::{
    Oauth2V3UserInfo, Oauth2V3UserInfoEndpoint, Oauth2V3UserInfoEndpointError,
};

//
#[derive(Debug, Clone)]
pub struct GoogleUserInfoEndpoint {
    inner: Oauth2V3UserInfoEndpoint,
}
impl GoogleUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            inner: Oauth2V3UserInfoEndpoint::new(access_token),
        }
    }
}

impl Endpoint for GoogleUserInfoEndpoint {
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
        Ok(UserInfo::try_from(self.inner.parse_response(response)?)
            .map_err(EndpointParseResponseError::ToOutputFailed)?)
    }
}

//
impl From<Oauth2V3UserInfoEndpointError> for EndpointRenderRequestError {
    fn from(err: Oauth2V3UserInfoEndpointError) -> Self {
        match err {
            Oauth2V3UserInfoEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            Oauth2V3UserInfoEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<Oauth2V3UserInfoEndpointError> for EndpointParseResponseError {
    fn from(err: Oauth2V3UserInfoEndpointError) -> Self {
        match err {
            Oauth2V3UserInfoEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            Oauth2V3UserInfoEndpointError::DeResponseBodyFailed(err) => {
                Self::DeResponseBodyFailed(err)
            }
        }
    }
}

//
impl TryFrom<Oauth2V3UserInfo> for UserInfo {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(user_info: Oauth2V3UserInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            uid: user_info.sub.to_owned(),
            name: None,
            email: user_info.email.to_owned(),
            raw: serde_json::to_value(user_info)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
