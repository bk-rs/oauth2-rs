use std::error;

use oauth2_client::{
    extensions::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_oauth_user_info_endpoint::{
    OauthUserInfoEndpoint, OauthUserInfoEndpointError, OauthUserInfoResponseBodyOkJson,
};

//
#[derive(Debug, Clone)]
pub struct ZohoUserInfoEndpoint {
    inner: OauthUserInfoEndpoint,
}
impl ZohoUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            inner: OauthUserInfoEndpoint::new(access_token),
        }
    }
}

impl Endpoint for ZohoUserInfoEndpoint {
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
impl From<OauthUserInfoEndpointError> for EndpointRenderRequestError {
    fn from(err: OauthUserInfoEndpointError) -> Self {
        match err {
            OauthUserInfoEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            OauthUserInfoEndpointError::DeResponseBodyOkJsonFailed(err) => {
                Self::Other(Box::new(err))
            }
            OauthUserInfoEndpointError::ResponseBodyError(status_code, body) => {
                Self::Other(format!("status_code:{} body:{:?}", status_code, body).into())
            }
        }
    }
}
impl From<OauthUserInfoEndpointError> for EndpointParseResponseError {
    fn from(err: OauthUserInfoEndpointError) -> Self {
        match err {
            OauthUserInfoEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            OauthUserInfoEndpointError::DeResponseBodyOkJsonFailed(err) => {
                Self::DeResponseBodyFailed(err)
            }
            OauthUserInfoEndpointError::ResponseBodyError(status_code, body) => {
                Self::Other(format!("status_code:{} body:{:?}", status_code, body).into())
            }
        }
    }
}

//
struct UserInfoWrapper(UserInfo);
impl TryFrom<OauthUserInfoResponseBodyOkJson> for UserInfoWrapper {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(ok_json: OauthUserInfoResponseBodyOkJson) -> Result<Self, Self::Error> {
        Ok(Self(UserInfo {
            uid: ok_json.zuid.to_string(),
            name: Some(ok_json.display_name.to_owned()),
            email: Some(ok_json.email.to_owned()),
            raw: serde_json::to_value(ok_json)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        }))
    }
}
