use oauth2_client::{
    extensions::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_get_account_endpoint::{Account, GetAccountEndpoint, GetAccountEndpointError};

type Uid = String;

//
#[derive(Debug, Clone)]
pub struct DropboxUserInfoEndpoint {
    inner: GetAccountEndpoint,
    uid: Uid,
}
impl DropboxUserInfoEndpoint {
    pub fn new(
        access_token: impl AsRef<str>,
        account_id: impl AsRef<str>,
        uid: impl AsRef<str>,
    ) -> Self {
        Self {
            inner: GetAccountEndpoint::new(access_token, account_id),
            uid: uid.as_ref().to_owned(),
        }
    }
}

impl Endpoint for DropboxUserInfoEndpoint {
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
        UserInfoWrapper::try_from((self.uid.to_owned(), self.inner.parse_response(response)?))
            .map(|x| x.0)
            .map_err(EndpointParseResponseError::ToOutputFailed)
    }
}

//
impl From<GetAccountEndpointError> for EndpointRenderRequestError {
    fn from(err: GetAccountEndpointError) -> Self {
        match err {
            GetAccountEndpointError::SerResponseBodyFailed(err) => Self::Other(Box::new(err)),
            GetAccountEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            GetAccountEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<GetAccountEndpointError> for EndpointParseResponseError {
    fn from(err: GetAccountEndpointError) -> Self {
        match err {
            GetAccountEndpointError::SerResponseBodyFailed(err) => Self::Other(Box::new(err)),
            GetAccountEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            GetAccountEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
        }
    }
}

//
struct UserInfoWrapper(UserInfo);
impl TryFrom<(Uid, Account)> for UserInfoWrapper {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from((uid, account): (Uid, Account)) -> Result<Self, Self::Error> {
        Ok(Self(UserInfo {
            uid,
            name: None,
            email: account.email.to_owned(),
            raw: serde_json::to_value(account)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        }))
    }
}
