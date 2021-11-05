use std::{fmt, str};

use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, EndpointExecuteError, UserInfo, UserInfoEndpoint,
    },
    authorization_code_grant::{
        provider_ext::ProviderExtAuthorizationCodeGrantStringScopeWrapper, Flow,
        FlowBuildAuthorizationUrlError, FlowHandleCallbackError,
    },
    oauth2_core::types::State,
    re_exports::{AccessTokenResponseSuccessfulBody, Client, Url},
    Provider, ProviderExtAuthorizationCodeGrant,
};

#[derive(Clone)]
pub struct SigninFlow<C>
where
    C: Client,
{
    pub flow: Flow<C>,
    pub provider: Box<dyn ProviderExtAuthorizationCodeGrant<Scope = String>>,
    pub scopes: Option<Vec<String>>,
    pub user_info_endpoint: Box<dyn UserInfoEndpoint<String>>,
    pub client_with_user_info: C,
    _priv: (),
}
impl<C> SigninFlow<C>
where
    C: Client,
{
    pub fn new<P, UIEP>(
        client: C,
        provider: P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        user_info_endpoint: UIEP,
    ) -> Self
    where
        C: Clone,
        P: ProviderExtAuthorizationCodeGrant + Clone + 'static,
        <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
        UIEP: UserInfoEndpoint<String> + 'static,
    {
        Self {
            flow: Flow::new(client.clone()),
            provider: Box::new(ProviderExtAuthorizationCodeGrantStringScopeWrapper::new(
                provider,
            )),
            scopes: scopes
                .into()
                .map(|x| x.iter().map(|y| y.to_string()).collect()),
            user_info_endpoint: Box::new(user_info_endpoint),
            client_with_user_info: client.clone(),
            _priv: (),
        }
    }
}

impl<C> SigninFlow<C>
where
    C: Client + Send + Sync,
{
    pub fn build_authorization_url(
        &self,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError> {
        self.flow
            .build_authorization_url(self.provider.as_ref(), self.scopes.to_owned(), state)
    }

    pub fn build_authorization_url_with_custom_scopes(
        &self,
        custom_scopes: Vec<String>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError> {
        self.flow
            .build_authorization_url(self.provider.as_ref(), Some(custom_scopes), state)
    }

    pub async fn handle_callback(
        &self,
        query: impl AsRef<str>,
        state: impl Into<Option<State>>,
    ) -> SigninFlowHandleCallbackRet {
        let access_token = match self
            .flow
            .handle_callback_with_dyn(self.provider.as_ref(), query, state)
            .await
        {
            Ok(x) => x,
            Err(err) => return SigninFlowHandleCallbackRet::FlowHandleCallbackError(err),
        };

        let access_token_obtain_from = AccessTokenObtainFrom::AuthorizationCodeGrant;

        if !self
            .user_info_endpoint
            .can_execute(access_token_obtain_from, &access_token)
        {
            return SigninFlowHandleCallbackRet::Ok((access_token, None));
        }

        let user_info_endpoint_request = match self
            .user_info_endpoint
            .render_request(access_token_obtain_from, &access_token)
        {
            Ok(x) => x,
            Err(err) => {
                return SigninFlowHandleCallbackRet::FetchUserInfoError((
                    access_token,
                    EndpointExecuteError::RenderRequestError(err),
                ));
            }
        };

        let user_info_endpoint_response = match self
            .client_with_user_info
            .respond(user_info_endpoint_request)
            .await
        {
            Ok(x) => x,
            Err(err) => {
                return SigninFlowHandleCallbackRet::FetchUserInfoError((
                    access_token,
                    EndpointExecuteError::RespondFailed(Box::new(err)),
                ));
            }
        };

        let user_info = match self
            .user_info_endpoint
            .parse_response(user_info_endpoint_response)
        {
            Ok(x) => x,
            Err(err) => {
                return SigninFlowHandleCallbackRet::FetchUserInfoError((
                    access_token,
                    EndpointExecuteError::ParseResponseError(err),
                ));
            }
        };

        SigninFlowHandleCallbackRet::Ok((access_token, Some(user_info)))
    }
}

#[derive(Debug)]
pub enum SigninFlowHandleCallbackRet {
    Ok((AccessTokenResponseSuccessfulBody<String>, Option<UserInfo>)),
    FetchUserInfoError(
        (
            AccessTokenResponseSuccessfulBody<String>,
            EndpointExecuteError,
        ),
    ),
    FlowHandleCallbackError(FlowHandleCallbackError),
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::HashMap, error};

    use oauth2_github::{GithubProviderWithWebApplication, GithubScope, GithubUserInfoEndpoint};
    use oauth2_google::{GoogleProviderForWebServerApps, GoogleScope, GoogleUserInfoEndpoint};

    use http_api_isahc_client::IsahcClient;

    #[test]
    fn test_build_authorization_url() -> Result<(), Box<dyn error::Error>> {
        let mut map = HashMap::new();
        map.insert(
            "github",
            SigninFlow::new(
                IsahcClient::new()?,
                GithubProviderWithWebApplication::new(
                    "client_id".to_owned(),
                    "client_secret".to_owned(),
                    "https://client.example.com/cb".parse()?,
                )?,
                vec![GithubScope::User],
                GithubUserInfoEndpoint,
            ),
        );
        map.insert(
            "google",
            SigninFlow::new(
                IsahcClient::new()?,
                GoogleProviderForWebServerApps::new(
                    "client_id".to_owned(),
                    "client_secret".to_owned(),
                    "https://client.example.com/cb".parse()?,
                )?,
                vec![GoogleScope::Email],
                GoogleUserInfoEndpoint,
            ),
        );

        let github_auth_url = map
            .get("github")
            .unwrap()
            .build_authorization_url_with_custom_scopes(
                vec![GithubScope::User.to_string(), "custom".to_owned()],
                "STATE".to_owned(),
            )?;
        println!("github_auth_url {}", github_auth_url);

        let google_auth_url = map.get("google").unwrap().build_authorization_url(None)?;
        println!("google_auth_url {}", google_auth_url);

        Ok(())
    }

    #[tokio::test]
    async fn test_handle_callback() -> Result<(), Box<dyn error::Error>> {
        let mut map = HashMap::new();
        map.insert(
            "github",
            SigninFlow::new(
                IsahcClient::new()?,
                GithubProviderWithWebApplication::new(
                    "client_id".to_owned(),
                    "client_secret".to_owned(),
                    "https://client.example.com/cb".parse()?,
                )?,
                vec![GithubScope::User],
                GithubUserInfoEndpoint,
            ),
        );

        let _ = map
            .get("github")
            .unwrap()
            .handle_callback("code=CODE&state=STATE", "xxx".to_owned())
            .await;

        Ok(())
    }
}
