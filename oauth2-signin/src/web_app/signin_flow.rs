use std::fmt;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, EndpointExecuteError, EndpointOutputObtainFrom,
        EndpointParseResponseError, UserInfoEndpoint,
    },
    authorization_code_grant::{
        provider_ext::ProviderExtAuthorizationCodeGrantStringScopeWrapper, Flow,
        FlowBuildAuthorizationUrlError,
    },
    oauth2_core::types::State,
    re_exports::{Client, Url},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use super::SigninFlowHandleCallbackRet;

//
//
//
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
}
impl<C> fmt::Debug for SigninFlow<C>
where
    C: Client + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigninFlow")
            .field("flow", &self.flow)
            .field("provider", &self.provider)
            .field("scopes", &self.scopes)
            .field("user_info_endpoint", &self.user_info_endpoint)
            .field("client_with_user_info", &self.client_with_user_info)
            .finish()
    }
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
            client_with_user_info: client,
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

        match self
            .user_info_endpoint
            .obtain_from(access_token_obtain_from, &access_token)
        {
            EndpointOutputObtainFrom::None => {
                return SigninFlowHandleCallbackRet::OkButUserInfoNone(access_token);
            }
            EndpointOutputObtainFrom::Build => {
                match self
                    .user_info_endpoint
                    .build(access_token_obtain_from, &access_token)
                {
                    Ok(user_info) => {
                        return SigninFlowHandleCallbackRet::Ok((access_token, user_info));
                    }
                    Err(err) => {
                        return SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
                            access_token,
                            EndpointExecuteError::ParseResponseError(
                                EndpointParseResponseError::Other(err.to_string()),
                            ),
                        ));
                    }
                };
            }
            EndpointOutputObtainFrom::Respond => {}
        }

        let user_info_endpoint_request = match self
            .user_info_endpoint
            .render_request(access_token_obtain_from, &access_token)
        {
            Ok(x) => x,
            Err(err) => {
                return SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
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
                return SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
                    access_token,
                    EndpointExecuteError::RespondFailed(err.to_string()),
                ));
            }
        };

        let user_info = match self.user_info_endpoint.parse_response(
            access_token_obtain_from,
            &access_token,
            user_info_endpoint_response,
        ) {
            Ok(x) => x,
            Err(err) => {
                return SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
                    access_token,
                    EndpointExecuteError::ParseResponseError(err),
                ));
            }
        };

        SigninFlowHandleCallbackRet::Ok((access_token, user_info))
    }
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

        //
        println!("{:?}", map);

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
