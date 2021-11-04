use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, EndpointExecuteError, UserInfo, UserInfoEndpoint,
    },
    re_exports::{
        async_trait, serde_json, AccessTokenResponseSuccessfulBody, Client,
        ClientRespondEndpointError,
    },
};

use super::internal_user_endpoint::{User, UserEndpoint, UserEndpointError};

//
pub struct GithubUserInfoEndpoint;

#[async_trait]
impl UserInfoEndpoint for GithubUserInfoEndpoint {
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
        let endpoint = UserEndpoint::new(&access_token.access_token);

        let user = client
            .respond_endpoint(&endpoint)
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    EndpointExecuteError::RespondFailed(Box::new(err))
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => err.into(),
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => err.into(),
            })?;

        Ok(UserInfo::try_from(user).map_err(EndpointExecuteError::ToUserInfoFailed)?)
    }
}

//
impl From<UserEndpointError> for EndpointExecuteError {
    fn from(err: UserEndpointError) -> Self {
        match err {
            UserEndpointError::MakeRequestFailed(err) => Self::MakeRequestFailed(err),
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
