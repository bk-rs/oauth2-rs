use std::error;

use oauth2_client::{
    additional_endpoints::{AccessTokenObtainFrom, UserInfo, UserInfoEndpoint},
    re_exports::{
        async_trait, serde_json, thiserror, AccessTokenResponseSuccessfulBody, Client,
        ClientRespondEndpointError, HttpError, SerdeJsonError,
    },
};

use super::internal_oauth2_v3_user_info_endpoint::{
    Oauth2V3UserInfo, Oauth2V3UserInfoEndpoint, Oauth2V3UserInfoEndpointError,
};

//
pub struct GoogleUserInfoEndpoint;

#[async_trait]
impl UserInfoEndpoint for GoogleUserInfoEndpoint {
    type Output = Oauth2V3UserInfo;
    type Error = GoogleUserInfoEndpointError;

    fn can_execute(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<String>,
    ) -> bool {
        true
    }

    async fn execute<C1, C2>(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<String>,
        client: &C1,
        _: &C2,
    ) -> Result<Self::Output, Self::Error>
    where
        C1: Client + Send + Sync,
        C2: Client + Send + Sync,
    {
        let endpoint = Oauth2V3UserInfoEndpoint::new(&access_token.access_token);

        let output = client
            .respond_endpoint(&endpoint)
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    GoogleUserInfoEndpointError::RespondFailed(Box::new(err))
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => err.into(),
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => err.into(),
            })?;

        Ok(output)
    }
}

//
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
impl From<Oauth2V3UserInfoEndpointError> for GoogleUserInfoEndpointError {
    fn from(err: Oauth2V3UserInfoEndpointError) -> Self {
        match err {
            Oauth2V3UserInfoEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
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
                .ok_or_else(|| "".to_owned())?,
        })
    }
}
