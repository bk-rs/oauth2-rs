use std::{collections::HashMap, fmt, str};

use oauth2_client::{
    additional_endpoints::UserInfoEndpoint,
    authorization_code_grant::{Flow, FlowBuildAuthorizationUrlError},
    oauth2_core::types::State,
    re_exports::{ClientId, ClientSecret, RedirectUri, Url},
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
    pub fn insert(&mut self, name: impl AsRef<str>, signin_flow: SigninFlow) -> Result<(), ()> {
        self.inner
            .insert(name.as_ref().to_owned(), signin_flow)
            .map(|_| ())
            .ok_or_else(|| ())
    }
    pub fn get(&self, name: impl AsRef<str>) -> Option<&SigninFlow> {
        self.inner.get(name.as_ref())
    }
}

pub struct XProviderWithWebApplicationWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    inner: P,
}

impl<P> XProviderWithWebApplicationWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for XProviderWithWebApplicationWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    type Scope = String;

    fn client_id(&self) -> Option<&ClientId> {
        self.inner.client_id()
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        self.inner.client_secret()
    }

    fn token_endpoint_url(&self) -> &Url {
        self.inner.token_endpoint_url()
    }
}
impl<P> ProviderExtAuthorizationCodeGrant for XProviderWithWebApplicationWrapper<P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        self.inner.redirect_uri()
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        self.inner
            .scopes_default()
            .map(|x| x.into_iter().map(|y| y.to_string()).collect())
    }

    fn authorization_endpoint_url(&self) -> &Url {
        self.inner.authorization_endpoint_url()
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
            provider: Box::new(XProviderWithWebApplicationWrapper::new(provider)),
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
