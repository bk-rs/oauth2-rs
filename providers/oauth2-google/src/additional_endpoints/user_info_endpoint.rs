use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, EndpointExecuteError, UserInfo, UserInfoEndpoint,
    },
    re_exports::{
        async_trait, serde_json, AccessTokenResponseSuccessfulBody, Client,
        ClientRespondEndpointError,
    },
};

use super::internal_oauth2_v3_user_info_endpoint::{
    Oauth2V3UserInfo, Oauth2V3UserInfoEndpoint, Oauth2V3UserInfoEndpointError,
};

//
pub struct GoogleUserInfoEndpoint;

#[async_trait]
impl UserInfoEndpoint for GoogleUserInfoEndpoint {
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
    ) -> Result<UserInfo, EndpointExecuteError>
    where
        C1: Client + Send + Sync,
        C2: Client + Send + Sync,
    {
        let endpoint = Oauth2V3UserInfoEndpoint::new(&access_token.access_token);

        let user_info = client
            .respond_endpoint(&endpoint)
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    EndpointExecuteError::RespondFailed(Box::new(err))
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => err.into(),
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => err.into(),
            })?;

        Ok(UserInfo::try_from(user_info).map_err(EndpointExecuteError::ToUserInfoFailed)?)
    }
}

//
impl From<Oauth2V3UserInfoEndpointError> for EndpointExecuteError {
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
                .ok_or_else(|| "unreachable".to_owned())?,
        })
    }
}
