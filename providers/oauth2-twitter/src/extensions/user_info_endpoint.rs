use std::error;

use oauth2_client::{
    extensions::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_users_me_endpoint::{User, UsersMeEndpoint, UsersMeEndpointError};

//
#[derive(Debug, Clone)]
pub struct TwitterUserInfoEndpoint {
    inner: UsersMeEndpoint,
}
impl TwitterUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            inner: UsersMeEndpoint::new(access_token),
        }
    }
}

impl Endpoint for TwitterUserInfoEndpoint {
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
impl From<UsersMeEndpointError> for EndpointRenderRequestError {
    fn from(err: UsersMeEndpointError) -> Self {
        match err {
            UsersMeEndpointError::UrlParseFailed(err) => Self::Other(Box::new(err)),
            UsersMeEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            UsersMeEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
            UsersMeEndpointError::ResponseBodyIsFail(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<UsersMeEndpointError> for EndpointParseResponseError {
    fn from(err: UsersMeEndpointError) -> Self {
        match err {
            UsersMeEndpointError::UrlParseFailed(err) => Self::Other(Box::new(err)),
            UsersMeEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            UsersMeEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
            UsersMeEndpointError::ResponseBodyIsFail(err) => Self::Other(Box::new(err)),
        }
    }
}

//
struct UserInfoWrapper(UserInfo);
impl TryFrom<User> for UserInfoWrapper {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        Ok(Self(UserInfo {
            uid: user.id.to_owned(),
            name: Some(user.username.to_owned()),
            email: None,
            raw: serde_json::to_value(user)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        }))
    }
}
