use std::{collections::HashMap, fmt, str};

use oauth2_client::{
    additional_endpoints::UserInfoEndpoint,
    authorization_code_grant::{
        provider_ext::ProviderExtAuthorizationCodeGrantStringScopeWrapper, Flow,
        FlowBuildAuthorizationUrlError,
    },
    oauth2_core::types::State,
    re_exports::Url,
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::HttpClient;

pub struct SigninFlowMap {
    inner: HashMap<String, SigninFlow>,
}
impl SigninFlowMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        name: impl AsRef<str>,
        signin_flow: SigninFlow,
    ) -> Result<(), &'static str> {
        match self.inner.insert(name.as_ref().to_owned(), signin_flow) {
            Some(_) => Err("name exists"),
            None => Ok(()),
        }
    }
    pub fn get(&self, name: impl AsRef<str>) -> Option<&SigninFlow> {
        self.inner.get(name.as_ref())
    }
}

pub struct SigninFlow {
    pub flow: Flow<HttpClient>,
    pub provider: Box<dyn ProviderExtAuthorizationCodeGrant<Scope = String>>,
    pub scopes: Option<Vec<String>>,
    pub user_info_endpoint: Box<dyn UserInfoEndpoint>,
    pub client_with_user_info: HttpClient,
    pub another_client_with_user_info: HttpClient,
    _priv: (),
}
impl SigninFlow {
    pub fn new<P, UIEP>(
        client: HttpClient,
        provider: P,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        user_info_endpoint: UIEP,
    ) -> Self
    where
        P: ProviderExtAuthorizationCodeGrant + 'static,
        <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
        UIEP: UserInfoEndpoint + 'static,
    {
        Self {
            flow: Flow::new(client.clone()),
            provider: Box::new(ProviderExtAuthorizationCodeGrantStringScopeWrapper::new(
                provider,
            )),
            scopes: scopes
                .into()
                .map(|x| x.into_iter().map(|y| y.to_string()).collect()),
            user_info_endpoint: Box::new(user_info_endpoint),
            client_with_user_info: client.clone(),
            another_client_with_user_info: client.clone(),
            _priv: (),
        }
    }
}

impl SigninFlow {
    pub fn build_authorization_url(
        &self,
        scopes: impl Into<Option<Vec<String>>>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError> {
        self.flow
            .build_authorization_url_with_dyn_provider(self.provider.as_ref(), scopes, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use oauth2_github::{GithubProviderWithWebApplication, GithubScope, GithubUserInfoEndpoint};
    use oauth2_google::{GoogleProviderForWebServerApps, GoogleScope, GoogleUserInfoEndpoint};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let mut map = SigninFlowMap::new();
        map.insert(
            "github",
            SigninFlow::new(
                HttpClient::new()?,
                GithubProviderWithWebApplication::new(
                    "client_id".to_owned(),
                    "client_secret".to_owned(),
                    "https://client.example.com/cb".parse()?,
                )?,
                vec![GithubScope::User],
                GithubUserInfoEndpoint,
            ),
        )?;
        map.insert(
            "google",
            SigninFlow::new(
                HttpClient::new()?,
                GoogleProviderForWebServerApps::new(
                    "client_id".to_owned(),
                    "client_secret".to_owned(),
                    "https://client.example.com/cb".parse()?,
                )?,
                vec![GoogleScope::Email],
                GoogleUserInfoEndpoint,
            ),
        )?;

        let github_auth_url = map.get("github").unwrap().build_authorization_url(
            vec!["foo".to_owned(), "bar".to_owned()],
            "STATE".to_owned(),
        )?;
        println!("github_auth_url {}", github_auth_url);

        let google_auth_url = map
            .get("google")
            .unwrap()
            .build_authorization_url(None, None)?;
        println!("google_auth_url {}", google_auth_url);

        Ok(())
    }
}
