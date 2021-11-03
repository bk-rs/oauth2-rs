use std::{error, fmt, str};

use oauth2_client::{
    provider::{
        http::header::{ACCEPT, AUTHORIZATION},
        http_api_endpoint::MIME_APPLICATION_JSON,
        serde::{Deserialize, Serialize},
        serde_json, thiserror, Body, HttpError, Request, Response, SerdeJsonError,
    },
    user_info::provider_ext::{
        async_trait, AccessTokenResponseSuccessfulBody, AccessTokenResponseSuccessfulBodySource,
        Client, ClientRespondEndpointError, Endpoint, ProviderExtUserInfo, UserInfo,
    },
    Provider,
};

pub const USER_INFO_URL: &str = "https://www.googleapis.com/oauth2/v3/userinfo";

//
pub struct GoogleOauth2V3UserInfoEndpoint<'a, P>
where
    P: Provider,
    <P::Scope as str::FromStr>::Err: fmt::Display,
{
    pub token_source: AccessTokenResponseSuccessfulBodySource,
    pub token: &'a AccessTokenResponseSuccessfulBody<<P as Provider>::Scope>,
}
impl<'a, P> GoogleOauth2V3UserInfoEndpoint<'a, P>
where
    P: Provider,
    <P::Scope as str::FromStr>::Err: fmt::Display,
{
    pub fn new(
        token_source: AccessTokenResponseSuccessfulBodySource,
        token: &'a AccessTokenResponseSuccessfulBody<<P as Provider>::Scope>,
    ) -> Self {
        Self {
            token_source,
            token,
        }
    }
}

impl<'a, P> Endpoint for GoogleOauth2V3UserInfoEndpoint<'a, P>
where
    P: Provider,
    <P::Scope as str::FromStr>::Err: fmt::Display,
{
    type RenderRequestError = GoogleUserInfoEndpointError;

    type ParseResponseOutput = GoogleOauth2V3UserInfo;
    type ParseResponseError = GoogleUserInfoEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(USER_INFO_URL)
            .header(
                AUTHORIZATION,
                format!("Bearer {}", &self.token.access_token),
            )
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(GoogleUserInfoEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<GoogleOauth2V3UserInfo>(&response.body())
            .map_err(GoogleUserInfoEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GoogleOauth2V3UserInfo {
    pub sub: String,
    pub picture: String,
    pub email: String,
}
impl UserInfo for GoogleOauth2V3UserInfo {
    fn uid(&self) -> String {
        self.sub.to_owned()
    }

    fn name(&self) -> Option<String> {
        None
    }

    fn email(&self) -> Option<String> {
        Some(self.email.to_owned())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GoogleUserInfoEndpointError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("RespondFailed {0}")]
    RespondFailed(Box<dyn error::Error>),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}

//
//
//
#[cfg(feature = "with-authorization-code-grant")]
#[async_trait]
impl ProviderExtUserInfo for super::authorization_code_grant::GoogleProviderForWebServerApps {
    type Output = GoogleOauth2V3UserInfo;

    type Error = GoogleUserInfoEndpointError;

    async fn fetch_user_info<C2, C3>(
        &self,
        token_source: AccessTokenResponseSuccessfulBodySource,
        token: &AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
        client: &C2,
        _: &C3,
    ) -> Result<Self::Output, Self::Error>
    where
        <Self::Scope as str::FromStr>::Err: fmt::Display,
        Self::Scope: Send + Sync,
        C2: Client + Send + Sync,
        C3: Client + Send + Sync,
    {
        let endpoint = GoogleOauth2V3UserInfoEndpoint::<Self>::new(token_source, token);

        let user_info = client
            .respond_endpoint(&endpoint)
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    GoogleUserInfoEndpointError::RespondFailed(Box::new(err))
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => err,
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => err,
            })?;

        Ok(user_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_user_info() {
        match serde_json::from_str::<GoogleOauth2V3UserInfo>(include_str!(
            "../tests/response_body_json_files/oauth2_v3.json"
        )) {
            Ok(user_info) => {
                assert_eq!(user_info.uid(), "110578243643543721809");
                assert_eq!(user_info.name(), None);
                assert_eq!(user_info.email(), Some("vkill.net@gmail.com".to_owned()));
            }
            Err(err) => panic!("{}", err),
        }
    }
}
