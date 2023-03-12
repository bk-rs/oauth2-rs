use oauth2_client::{
    extensions::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_userinfo_endpoint::{User, UserEndpoint, UserEndpointError};

//
#[derive(Debug, Clone)]
pub struct YahooUserInfoEndpoint {
    inner: UserEndpoint,
}
impl YahooUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            inner: UserEndpoint::new(access_token),
        }
    }
}

impl Endpoint for YahooUserInfoEndpoint {
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
impl From<UserEndpointError> for EndpointRenderRequestError {
    fn from(err: UserEndpointError) -> Self {
        match err {
            UserEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            UserEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<UserEndpointError> for EndpointParseResponseError {
    fn from(err: UserEndpointError) -> Self {
        match err {
            UserEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            UserEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
        }
    }
}

//
impl TryFrom<User> for UserInfo {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        Ok(Self {
            uid: user.sub.to_string(),
            name: Some(user.name.to_owned()),
            email: Some(user.email.to_owned()),
            raw: serde_json::to_value(user)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
