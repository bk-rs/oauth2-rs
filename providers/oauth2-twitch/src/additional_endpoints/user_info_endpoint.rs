use std::error;

use oauth2_client::{
    additional_endpoints::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_users_endpoint::{Users, UsersEndpoint, UsersEndpointError};

//
#[derive(Debug, Clone)]
pub struct TwitchUserInfoEndpoint {
    inner: UsersEndpoint,
}
impl TwitchUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>, client_id: impl AsRef<str>) -> Self {
        Self {
            inner: UsersEndpoint::new(access_token, client_id),
        }
    }
}

impl Endpoint for TwitchUserInfoEndpoint {
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
impl From<UsersEndpointError> for EndpointRenderRequestError {
    fn from(err: UsersEndpointError) -> Self {
        match err {
            UsersEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            UsersEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<UsersEndpointError> for EndpointParseResponseError {
    fn from(err: UsersEndpointError) -> Self {
        match err {
            UsersEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            UsersEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
        }
    }
}

//
impl TryFrom<Users> for UserInfo {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(users: Users) -> Result<Self, Self::Error> {
        let user = users.data.first().cloned().ok_or("not found user")?;

        Ok(Self {
            uid: user.id.to_owned(),
            name: Some(user.login.to_owned()),
            email: user.email.to_owned(),
            raw: serde_json::to_value(user)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
