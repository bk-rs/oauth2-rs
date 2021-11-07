use std::fmt;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenObtainFrom, EndpointBuilder, EndpointExecuteError, UserInfoObtainRet,
    },
    authorization_code_grant::{
        provider_ext::ProviderExtAuthorizationCodeGrantStringScopeWrapper, Flow,
    },
    oauth2_core::types::State,
    re_exports::{Client, ClientRespondEndpointError, Url},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use super::{SigninFlowBuildAuthorizationUrlError, SigninFlowHandleCallbackRet};

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
    pub endpoint_builder: Box<dyn EndpointBuilder<String> + Send + Sync>,
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
            .field("endpoint_builder", &self.endpoint_builder)
            .field("client_with_user_info", &self.client_with_user_info)
            .finish()
    }
}

impl<C> SigninFlow<C>
where
    C: Client,
{
    pub fn new<P, EPB>(
        client: C,
        provider: P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        endpoint_builder: EPB,
    ) -> Self
    where
        C: Clone,
        P: ProviderExtAuthorizationCodeGrant + Clone + Send + Sync + 'static,
        EPB: EndpointBuilder<String> + Send + Sync + 'static,
    {
        Self {
            flow: Flow::new(client.clone()),
            provider: Box::new(ProviderExtAuthorizationCodeGrantStringScopeWrapper::new(
                provider,
            )),
            scopes: scopes
                .into()
                .map(|x| x.iter().map(|y| y.to_string()).collect()),
            endpoint_builder: Box::new(endpoint_builder),
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
    ) -> Result<Url, SigninFlowBuildAuthorizationUrlError> {
        self.flow
            .build_authorization_url(self.provider.as_ref(), self.scopes.to_owned(), state)
    }

    pub fn build_authorization_url_with_custom_scopes(
        &self,
        custom_scopes: Vec<String>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, SigninFlowBuildAuthorizationUrlError> {
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
            .handle_callback(self.provider.as_ref(), query, state)
            .await
        {
            Ok(x) => x,
            Err(err) => return SigninFlowHandleCallbackRet::FlowHandleCallbackError(err),
        };

        let access_token_obtain_from = AccessTokenObtainFrom::AuthorizationCodeGrant;

        match self
            .endpoint_builder
            .user_info_obtain(access_token_obtain_from, &access_token)
        {
            UserInfoObtainRet::None => SigninFlowHandleCallbackRet::OkButUserInfoNone(access_token),
            UserInfoObtainRet::Static(user_info) => {
                SigninFlowHandleCallbackRet::Ok((access_token, user_info))
            }
            UserInfoObtainRet::Respond(user_info_endpoint) => {
                match self
                    .client_with_user_info
                    .respond_dyn_endpoint(&user_info_endpoint)
                    .await
                {
                    Ok(user_info) => SigninFlowHandleCallbackRet::Ok((access_token, user_info)),
                    Err(err) => match err {
                        ClientRespondEndpointError::RespondFailed(err) => {
                            SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
                                access_token,
                                EndpointExecuteError::RespondFailed(err.to_string()),
                            ))
                        }
                        ClientRespondEndpointError::EndpointRenderRequestFailed(err) => {
                            SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
                                access_token,
                                EndpointExecuteError::RenderRequestError(err),
                            ))
                        }
                        ClientRespondEndpointError::EndpointParseResponseFailed(err) => {
                            SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
                                access_token,
                                EndpointExecuteError::ParseResponseError(err),
                            ))
                        }
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::HashMap, error};

    use oauth2_github::{GithubEndpointBuilder, GithubProviderWithWebApplication, GithubScope};
    use oauth2_google::{GoogleEndpointBuilder, GoogleProviderForWebServerApps, GoogleScope};

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
                GithubEndpointBuilder,
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
                GoogleEndpointBuilder,
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
                GithubEndpointBuilder,
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
