use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, EndpointParseResponseError, EndpointRenderRequestError, UserInfo,
        UserInfoEndpoint,
    },
    re_exports::{
        serde_json, AccessTokenResponseSuccessfulBody, Body, Endpoint, Request, Response, Scope,
    },
};

use super::internal_user_endpoint::{User, UserEndpoint, UserEndpointError};

//
#[derive(Debug, Clone)]
pub struct GithubUserInfoEndpoint;

impl<SCOPE> UserInfoEndpoint<SCOPE> for GithubUserInfoEndpoint
where
    SCOPE: Scope,
{
    fn can_execute(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> bool {
        true
    }

    fn render_request(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<Request<Body>, EndpointRenderRequestError> {
        let endpoint = UserEndpoint::new(&access_token.access_token);

        endpoint.render_request().map_err(Into::into)
    }

    fn parse_response(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
        response: Response<Body>,
    ) -> Result<UserInfo, EndpointParseResponseError> {
        let endpoint = UserEndpoint::new("");

        let user = endpoint.parse_response(response)?;

        Ok(UserInfo::try_from(user).map_err(EndpointParseResponseError::ToOutputFailed)?)
    }
}

//
impl From<UserEndpointError> for EndpointRenderRequestError {
    fn from(err: UserEndpointError) -> Self {
        match err {
            UserEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            UserEndpointError::DeResponseBodyFailed(err) => Self::Other(err.to_string()),
        }
    }
}
impl From<UserEndpointError> for EndpointParseResponseError {
    fn from(err: UserEndpointError) -> Self {
        match err {
            UserEndpointError::MakeRequestFailed(err) => Self::Other(err.to_string()),
            UserEndpointError::DeResponseBodyFailed(err) => Self::DeResponseBodyFailed(err),
        }
    }
}

//
impl TryFrom<User> for UserInfo {
    type Error = String;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        Ok(Self {
            uid: user.id.to_string(),
            name: Some(user.name.to_string()),
            email: Some(user.email.to_string()),
            raw: serde_json::to_value(user)
                .map(|x| x.as_object().cloned())
                .map_err(|err| err.to_string())?
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
