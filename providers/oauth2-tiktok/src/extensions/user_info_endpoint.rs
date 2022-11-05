use std::error;

use oauth2_client::{
    extensions::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_v2_user_info_endpoint::{
    UserObject, V2UserInfoEndpoint, V2UserInfoEndpointError,
};

//
#[derive(Debug, Clone)]
pub struct TiktokUserInfoEndpoint {
    inner: V2UserInfoEndpoint,
}
impl TiktokUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            inner: V2UserInfoEndpoint::new(access_token),
        }
    }
}

impl Endpoint for TiktokUserInfoEndpoint {
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
impl From<V2UserInfoEndpointError> for EndpointRenderRequestError {
    fn from(err: V2UserInfoEndpointError) -> Self {
        match err {
            V2UserInfoEndpointError::UrlParseFailed(err) => Self::Other(Box::new(err)),
            V2UserInfoEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            V2UserInfoEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
            V2UserInfoEndpointError::ResponseBodyIsError(err) => Self::Other(Box::new(err)),
            V2UserInfoEndpointError::ResponseBodyDataInvalid(_) => {
                Self::Other("DataInvalid".into())
            }
        }
    }
}
impl From<V2UserInfoEndpointError> for EndpointParseResponseError {
    fn from(err: V2UserInfoEndpointError) -> Self {
        match err {
            V2UserInfoEndpointError::UrlParseFailed(err) => Self::Other(Box::new(err)),
            V2UserInfoEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            V2UserInfoEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
            V2UserInfoEndpointError::ResponseBodyIsError(err) => Self::Other(Box::new(err)),
            V2UserInfoEndpointError::ResponseBodyDataInvalid(_) => {
                Self::Other("DataInvalid".into())
            }
        }
    }
}

//
struct UserInfoWrapper(UserInfo);
impl TryFrom<UserObject> for UserInfoWrapper {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(user: UserObject) -> Result<Self, Self::Error> {
        Ok(Self(UserInfo {
            uid: user.open_id.to_owned(),
            name: user.display_name.to_owned(),
            email: None,
            raw: serde_json::to_value(user)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        }))
    }
}
