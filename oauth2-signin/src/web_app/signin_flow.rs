use std::fmt;

use oauth2_client::{
    authorization_code_grant::{
        provider_ext::{
            ProviderExtAuthorizationCodeGrantOidcSupportType,
            ProviderExtAuthorizationCodeGrantStringScopeWrapper,
        },
        Flow,
    },
    extensions::{
        AuthorizationCodeGrantInfo, BuilderObtainUserInfoOutput, EndpointExecuteError, GrantInfo,
    },
    oauth2_core::types::{CodeChallenge, CodeChallengeMethod, CodeVerifier, Nonce, State},
    re_exports::{Client, ClientRespondEndpointError, Url},
    ExtensionsBuilder, Provider, ProviderExtAuthorizationCodeGrant,
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
    pub provider: Box<dyn ProviderExtAuthorizationCodeGrant<Scope = String> + Send + Sync>,
    pub scopes: Option<Vec<String>>,
    pub extensions_builder: Box<dyn ExtensionsBuilder<String> + Send + Sync>,
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
            .field("extensions_builder", &self.extensions_builder)
            .field("client_with_user_info", &self.client_with_user_info)
            .finish()
    }
}

impl<C> SigninFlow<C>
where
    C: Client,
{
    pub fn new<P, EB>(
        client: C,
        provider: P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        extensions_builder: EB,
    ) -> Self
    where
        C: Clone,
        P: ProviderExtAuthorizationCodeGrant + Clone + Send + Sync + 'static,
        EB: ExtensionsBuilder<String> + Send + Sync + 'static,
    {
        Self {
            flow: Flow::new(client.clone()),
            provider: Box::new(ProviderExtAuthorizationCodeGrantStringScopeWrapper::new(
                provider,
            )),
            scopes: scopes
                .into()
                .map(|x| x.iter().map(|y| y.to_string()).collect()),
            extensions_builder: Box::new(extensions_builder),
            client_with_user_info: client,
        }
    }
}

impl<C> SigninFlow<C>
where
    C: Client + Send + Sync,
{
    pub fn is_oidc_support(&self) -> bool {
        self.provider.oidc_support_type().map(|x| {
            matches!(
                x,
                ProviderExtAuthorizationCodeGrantOidcSupportType::Yes
                    | ProviderExtAuthorizationCodeGrantOidcSupportType::Force
            )
        }) == Some(true)
    }

    pub fn build_authorization_url(
        &self,
        state: impl Into<Option<State>>,
        code_challenge: impl Into<Option<(CodeChallenge, CodeChallengeMethod)>>,
    ) -> Result<Url, SigninFlowBuildAuthorizationUrlError> {
        self.build_authorization_url_with_oidc(state, code_challenge, None)
    }

    // OIDC
    pub fn build_authorization_url_with_oidc(
        &self,
        state: impl Into<Option<State>>,
        code_challenge: impl Into<Option<(CodeChallenge, CodeChallengeMethod)>>,
        nonce: impl Into<Option<Nonce>>,
    ) -> Result<Url, SigninFlowBuildAuthorizationUrlError> {
        self.flow.build_authorization_url_with_oidc(
            self.provider.as_ref(),
            self.scopes.to_owned(),
            state,
            code_challenge,
            nonce,
        )
    }

    pub async fn handle_callback(
        &self,
        query: impl AsRef<str>,
        state: impl Into<Option<State>>,
        code_verifier: impl Into<Option<CodeVerifier>>,
    ) -> SigninFlowHandleCallbackRet {
        self.handle_callback_with_oidc(query, state, code_verifier, None)
            .await
    }

    pub async fn handle_callback_with_oidc(
        &self,
        query: impl AsRef<str>,
        state: impl Into<Option<State>>,
        code_verifier: impl Into<Option<CodeVerifier>>,
        nonce: impl Into<Option<Nonce>>,
    ) -> SigninFlowHandleCallbackRet {
        let access_token = match self
            .flow
            .handle_callback_by_query(self.provider.as_ref(), query, state, code_verifier)
            .await
        {
            Ok(x) => x,
            Err(err) => return SigninFlowHandleCallbackRet::FlowHandleCallbackError(err),
        };

        let nonce = nonce.into();
        let grant_info = GrantInfo::AuthorizationCodeGrant(AuthorizationCodeGrantInfo {
            provider: self.provider.as_ref(),
            authorization_request_scopes: self.scopes.as_ref(),
            authorization_request_nonce: nonce.as_ref(),
        });

        match self
            .extensions_builder
            .obtain_user_info(grant_info, &access_token)
        {
            Ok(BuilderObtainUserInfoOutput::None) => {
                SigninFlowHandleCallbackRet::OkButUserInfoNone(access_token)
            }
            Ok(BuilderObtainUserInfoOutput::Static(user_info)) => {
                SigninFlowHandleCallbackRet::Ok((access_token, user_info))
            }
            Ok(BuilderObtainUserInfoOutput::Respond(user_info_endpoint)) => {
                match self
                    .client_with_user_info
                    .respond_dyn_endpoint(user_info_endpoint.as_ref())
                    .await
                {
                    Ok(user_info) => SigninFlowHandleCallbackRet::Ok((access_token, user_info)),
                    Err(err) => match err {
                        ClientRespondEndpointError::RespondFailed(err) => {
                            SigninFlowHandleCallbackRet::OkButUserInfoEndpointExecuteError((
                                access_token,
                                EndpointExecuteError::RespondFailed(Box::new(err)),
                            ))
                        }
                        ClientRespondEndpointError::EndpointRenderRequestFailed(err) => {
                            SigninFlowHandleCallbackRet::OkButUserInfoEndpointExecuteError((
                                access_token,
                                EndpointExecuteError::RenderRequestError(err),
                            ))
                        }
                        ClientRespondEndpointError::EndpointParseResponseFailed(err) => {
                            SigninFlowHandleCallbackRet::OkButUserInfoEndpointExecuteError((
                                access_token,
                                EndpointExecuteError::ParseResponseError(err),
                            ))
                        }
                    },
                }
            }
            Err(err) => SigninFlowHandleCallbackRet::OkButUserInfoObtainError((access_token, err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::HashMap, error};

    use oauth2_github::{GithubExtensionsBuilder, GithubProviderWithWebApplication, GithubScope};
    use oauth2_google::{GoogleExtensionsBuilder, GoogleProviderForWebServerApps, GoogleScope};

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
                GithubExtensionsBuilder,
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
                GoogleExtensionsBuilder,
            ),
        );

        let github_auth_url = map
            .get("github")
            .unwrap()
            .build_authorization_url("STATE".to_owned(), None)?;
        println!("github_auth_url {}", github_auth_url);

        let google_auth_url = map
            .get("google")
            .unwrap()
            .build_authorization_url(None, None)?;
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
                GithubExtensionsBuilder,
            ),
        );

        let _ = map
            .get("github")
            .unwrap()
            .handle_callback("code=CODE&state=STATE", "xxx".to_owned(), None)
            .await;

        Ok(())
    }
}
