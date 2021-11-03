use std::{collections::HashMap, fmt, str};

use oauth2_client::{
    authorization_code_grant::{
        provider_ext::AccessTokenResponseSuccessfulBody, Flow, FlowBuildAuthorizationUrlError,
        FlowHandleCallbackError,
    },
    oauth2_core::authorization_code_grant::authorization_request::State,
    provider::{DeserializeOwned, Serialize, Url},
    user_info::provider_ext::{AccessTokenResponseSuccessfulBodySource, Client},
    Provider, ProviderExtAuthorizationCodeGrant, ProviderExtUserInfo,
};

#[derive(Debug, Clone)]
pub struct SigninFlowMap<P, C1, C2, C3>
where
    C1: Client,
{
    inner: HashMap<String, SigninFlow<P, C1, C2, C3>>,
}
impl<P, C1, C2, C3> SigninFlowMap<P, C1, C2, C3>
where
    C1: Client,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        name: impl AsRef<str>,
        signin_provider: SigninFlow<P, C1, C2, C3>,
    ) -> Result<(), ()> {
        self.inner
            .insert(name.as_ref().to_owned(), signin_provider)
            .map(|_| ())
            .ok_or_else(|| ())
    }
    pub fn get(&self, name: impl AsRef<str>) -> Option<&SigninFlow<P, C1, C2, C3>> {
        self.inner.get(name.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct SigninFlow<P, C1, C2, C3>
where
    C1: Client,
{
    flow: Flow<C1>,
    provider: P,
    client_with_user_info: C2,
    another_client_with_user_info: C3,
}
impl<P, C1, C2, C3> SigninFlow<P, C1, C2, C3>
where
    C1: Client,
{
    pub fn new(
        flow: Flow<C1>,
        provider: P,
        client_with_user_info: C2,
        another_client_with_user_info: C3,
    ) -> Self {
        Self {
            flow,
            provider,
            client_with_user_info,
            another_client_with_user_info,
        }
    }
}

impl<P, C1, C2, C3> SigninFlow<P, C1, C2, C3>
where
    P: Provider + ProviderExtAuthorizationCodeGrant + ProviderExtUserInfo,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: Serialize,
    C1: Client,
    C2: Client,
    C3: Client,
{
    pub fn build_authorization_url(
        &self,
        scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError> {
        self.flow
            .build_authorization_url(&self.provider, scopes, state)
    }
}

impl<P, C1, C2, C3> SigninFlow<P, C1, C2, C3>
where
    P: Provider + ProviderExtAuthorizationCodeGrant + ProviderExtUserInfo + Send + Sync,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: Serialize + DeserializeOwned + Send + Sync,
    C1: Client + Send + Sync,
    C2: Client + Send + Sync,
    C3: Client + Send + Sync,
{
    pub async fn handle_callback(
        &self,
        query: impl AsRef<str>,
        state: impl Into<Option<State>>,
    ) -> SigninFlowRet<P> {
        let token = match self
            .flow
            .handle_callback(&self.provider, query, state)
            .await
        {
            Ok(x) => x,
            Err(err) => return SigninFlowRet::FlowHandleCallbackError(err),
        };

        let token_source = AccessTokenResponseSuccessfulBodySource::AuthorizationCodeGrant;

        let user_info = match self
            .provider
            .fetch_user_info(
                token_source,
                &token,
                &self.client_with_user_info,
                &self.another_client_with_user_info,
            )
            .await
        {
            Ok(x) => x,
            Err(err) => return SigninFlowRet::FetchUserInfoError((token, err)),
        };

        SigninFlowRet::Ok((token, user_info))
    }
}

pub enum SigninFlowRet<P>
where
    P: Provider + ProviderExtAuthorizationCodeGrant + ProviderExtUserInfo,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    Ok(
        (
            AccessTokenResponseSuccessfulBody<<P as Provider>::Scope>,
            <P as ProviderExtUserInfo>::Output,
        ),
    ),
    FetchUserInfoError(
        (
            AccessTokenResponseSuccessfulBody<<P as Provider>::Scope>,
            <P as ProviderExtUserInfo>::Error,
        ),
    ),
    FlowHandleCallbackError(FlowHandleCallbackError),
}
