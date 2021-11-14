use std::error;

use oauth2_client::{
    additional_endpoints::{EndpointParseResponseError, EndpointRenderRequestError, UserInfo},
    re_exports::{serde_json, Body, Endpoint, Request, Response},
};

use super::internal_graph_me_endpoint::{GraphMeEndpoint, GraphMeEndpointError, User};

pub type IgUserId = u64;

//
#[derive(Debug, Clone)]
pub struct InstagramUserInfoEndpoint {
    inner: GraphMeEndpoint,
    ig_user_id: IgUserId,
}
impl InstagramUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>, ig_user_id: IgUserId) -> Self {
        Self {
            inner: GraphMeEndpoint::new(access_token),
            ig_user_id,
        }
    }
}

impl Endpoint for InstagramUserInfoEndpoint {
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
        UserInfoWrapper::try_from((self.ig_user_id, self.inner.parse_response(response)?))
            .map(|x| x.0)
            .map_err(EndpointParseResponseError::ToOutputFailed)
    }
}

//
impl From<GraphMeEndpointError> for EndpointRenderRequestError {
    fn from(err: GraphMeEndpointError) -> Self {
        match err {
            GraphMeEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            GraphMeEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<GraphMeEndpointError> for EndpointParseResponseError {
    fn from(err: GraphMeEndpointError) -> Self {
        match err {
            GraphMeEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            GraphMeEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
        }
    }
}

//
struct UserInfoWrapper(UserInfo);
impl TryFrom<(IgUserId, User)> for UserInfoWrapper {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from((ig_user_id, user): (IgUserId, User)) -> Result<Self, Self::Error> {
        Ok(Self(UserInfo {
            uid: ig_user_id.to_string(),
            name: Some(user.username.to_owned()),
            email: None,
            raw: serde_json::to_value(user)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        }))
    }
}
