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

use crate::USER_INFO_URL;

//
pub struct GithubUserInfoEndpoint<'a, P>
where
    P: Provider,
    <P::Scope as str::FromStr>::Err: fmt::Display,
{
    pub token_source: AccessTokenResponseSuccessfulBodySource,
    pub token: &'a AccessTokenResponseSuccessfulBody<<P as Provider>::Scope>,
}
impl<'a, P> GithubUserInfoEndpoint<'a, P>
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

impl<'a, P> Endpoint for GithubUserInfoEndpoint<'a, P>
where
    P: Provider,
    <P::Scope as str::FromStr>::Err: fmt::Display,
{
    type RenderRequestError = GithubUserInfoEndpointError;

    type ParseResponseOutput = GithubUserInfo;
    type ParseResponseError = GithubUserInfoEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(USER_INFO_URL)
            .header(AUTHORIZATION, format!("token {}", &self.token.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(GithubUserInfoEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<GithubUserInfo>(&response.body())
            .map_err(GithubUserInfoEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GithubUserInfo {
    pub login: String,
    pub id: usize,
    pub name: String,
    pub email: String,
}
impl UserInfo for GithubUserInfo {
    fn uid(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> Option<String> {
        Some(self.name.to_owned())
    }

    fn email(&self) -> Option<String> {
        Some(self.email.to_owned())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GithubUserInfoEndpointError {
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
impl ProviderExtUserInfo for super::authorization_code_grant::GithubProviderWithWebApplication {
    type Output = GithubUserInfo;

    type Error = GithubUserInfoEndpointError;

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
        let endpoint = GithubUserInfoEndpoint::<Self>::new(token_source, token);

        let user_info = client
            .respond_endpoint(&endpoint)
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    GithubUserInfoEndpointError::RespondFailed(Box::new(err))
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => err,
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => err,
            })?;

        Ok(user_info)
    }
}
