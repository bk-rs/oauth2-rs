use oauth2_client::{
    extensions::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_get_user_account_endpoint::{
    GetUserAccountEndpoint, GetUserAccountEndpointError, UserAccount,
};

//
#[derive(Debug, Clone)]
pub struct PinterestUserInfoEndpoint {
    inner: GetUserAccountEndpoint,
}
impl PinterestUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            inner: GetUserAccountEndpoint::new(access_token),
        }
    }
}

impl Endpoint for PinterestUserInfoEndpoint {
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
impl From<GetUserAccountEndpointError> for EndpointRenderRequestError {
    fn from(err: GetUserAccountEndpointError) -> Self {
        match err {
            GetUserAccountEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            GetUserAccountEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<GetUserAccountEndpointError> for EndpointParseResponseError {
    fn from(err: GetUserAccountEndpointError) -> Self {
        match err {
            GetUserAccountEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            GetUserAccountEndpointError::DeResponseBodyFailed(err) => {
                Self::DeResponseBodyFailed(err)
            }
        }
    }
}

//
struct UserInfoWrapper(UserInfo);
impl TryFrom<UserAccount> for UserInfoWrapper {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(user_account: UserAccount) -> Result<Self, Self::Error> {
        Ok(Self(UserInfo {
            uid: user_account.username.to_owned(),
            name: Some(user_account.username.to_owned()),
            email: None,
            raw: serde_json::to_value(user_account)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        }))
    }
}
