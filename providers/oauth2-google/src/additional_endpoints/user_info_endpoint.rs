use std::{fmt, str};

use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, EndpointParseResponseError, EndpointRenderRequestError, UserInfo,
        UserInfoEndpoint,
    },
    re_exports::{
        serde_json, AccessTokenResponseSuccessfulBody, Body, Endpoint, Request, Response, Scope,
    },
};

use super::internal_oauth2_v3_user_info_endpoint::{
    Oauth2V3UserInfo, Oauth2V3UserInfoEndpoint, Oauth2V3UserInfoEndpointError,
};

//
#[derive(Debug, Clone)]
pub struct GoogleUserInfoEndpoint;

impl<SCOPE> UserInfoEndpoint<SCOPE> for GoogleUserInfoEndpoint
where
    SCOPE: Scope,
    <SCOPE as str::FromStr>::Err: fmt::Display,
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
        let endpoint = Oauth2V3UserInfoEndpoint::new(&access_token.access_token);

        endpoint.render_request().map_err(Into::into)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<UserInfo, EndpointParseResponseError> {
        let endpoint = Oauth2V3UserInfoEndpoint::new("");

        let user = endpoint.parse_response(response)?;

        Ok(UserInfo::try_from(user).map_err(EndpointParseResponseError::ToOutputFailed)?)
    }
}

//
impl From<Oauth2V3UserInfoEndpointError> for EndpointRenderRequestError {
    fn from(err: Oauth2V3UserInfoEndpointError) -> Self {
        match err {
            Oauth2V3UserInfoEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
            Oauth2V3UserInfoEndpointError::DeResponseBodyFailed(err) => Self::Other(Box::new(err)),
        }
    }
}
impl From<Oauth2V3UserInfoEndpointError> for EndpointParseResponseError {
    fn from(err: Oauth2V3UserInfoEndpointError) -> Self {
        match err {
            Oauth2V3UserInfoEndpointError::MakeRequestFailed(err) => Self::Other(Box::new(err)),
            Oauth2V3UserInfoEndpointError::DeResponseBodyFailed(err) => {
                Self::DeResponseBodyFailed(err)
            }
        }
    }
}

//
impl TryFrom<Oauth2V3UserInfo> for UserInfo {
    type Error = String;

    fn try_from(user_info: Oauth2V3UserInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            uid: user_info.sub.to_owned(),
            name: None,
            email: Some(user_info.email.to_string()),
            raw: serde_json::to_value(user_info)
                .map(|x| x.as_object().cloned())
                .map_err(|err| err.to_string())?
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
