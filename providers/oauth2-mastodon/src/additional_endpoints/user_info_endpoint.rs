use std::error;

use oauth2_client::{
    additional_endpoints::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response, UrlParseError},
};

use super::internal_accounts_verify_credentials_endpoint::{
    Account, AccountsVerifyCredentialsEndpoint, AccountsVerifyCredentialsEndpointError,
};

//
#[derive(Debug, Clone)]
pub struct MastodonUserInfoEndpoint {
    inner: AccountsVerifyCredentialsEndpoint,
}
impl MastodonUserInfoEndpoint {
    pub fn new(
        base_url: impl AsRef<str>,
        access_token: impl AsRef<str>,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            inner: AccountsVerifyCredentialsEndpoint::new(base_url, access_token)?,
        })
    }
}

impl Endpoint for MastodonUserInfoEndpoint {
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
impl From<AccountsVerifyCredentialsEndpointError> for EndpointRenderRequestError {
    fn from(err: AccountsVerifyCredentialsEndpointError) -> Self {
        match err {
            AccountsVerifyCredentialsEndpointError::MakeRequestFailed(err) => {
                Self::MakeRequestFailed(err)
            }
            AccountsVerifyCredentialsEndpointError::DeResponseBodyFailed(err) => {
                Self::Other(Box::new(err))
            }
        }
    }
}
impl From<AccountsVerifyCredentialsEndpointError> for EndpointParseResponseError {
    fn from(err: AccountsVerifyCredentialsEndpointError) -> Self {
        match err {
            AccountsVerifyCredentialsEndpointError::MakeRequestFailed(err) => {
                Self::Other(Box::new(err))
            }
            AccountsVerifyCredentialsEndpointError::DeResponseBodyFailed(err) => {
                Self::DeResponseBodyFailed(err)
            }
        }
    }
}

//
impl TryFrom<Account> for UserInfo {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(account: Account) -> Result<Self, Self::Error> {
        Ok(Self {
            uid: account.id.to_string(),
            name: Some(account.username.to_owned()),
            email: None,
            raw: serde_json::to_value(account)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
