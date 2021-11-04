#[cfg(feature = "with-http-api-isahc-client")]
pub(crate) use http_api_isahc_client::IsahcClient as HttpClient;
#[cfg(feature = "http-api-reqwest-client")]
pub(crate) use http_api_reqwest_client::ReqwestClient as HttpClient;

#[cfg(any(
    feature = "with-http-api-isahc-client",
    feature = "http-api-reqwest-client"
))]
mod inner;

#[cfg(any(
    feature = "with-http-api-isahc-client",
    feature = "http-api-reqwest-client"
))]
pub use inner::{SigninFlow, SigninFlowMap};

// impl<P, C1, C2, C3> SigninFlow<P, C1, C2, C3>
// where
//     P: Provider + ProviderExtAuthorizationCodeGrant + ProviderExtUserInfo,
//     <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
//     <P as Provider>::Scope: Serialize,
//     C1: Client,
//     C2: Client,
//     C3: Client,
// {
//     pub fn build_authorization_url(
//         &self,
//         scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>,
//         state: impl Into<Option<State>>,
//     ) -> Result<Url, FlowBuildAuthorizationUrlError> {
//         self.flow
//             .build_authorization_url(&self.provider, scopes, state)
//     }
// }

// impl<P, C1, C2, C3> SigninFlow<P, C1, C2, C3>
// where
//     P: Provider + ProviderExtAuthorizationCodeGrant + ProviderExtUserInfo + Send + Sync,
//     <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
//     <P as Provider>::Scope: Serialize + DeserializeOwned + Send + Sync,
//     C1: Client + Send + Sync,
//     C2: Client + Send + Sync,
//     C3: Client + Send + Sync,
// {
//     pub async fn handle_callback(
//         &self,
//         query: impl AsRef<str>,
//         state: impl Into<Option<State>>,
//     ) -> SigninFlowRet<P> {
//         let token = match self
//             .flow
//             .handle_callback(&self.provider, query, state)
//             .await
//         {
//             Ok(x) => x,
//             Err(err) => return SigninFlowRet::FlowHandleCallbackError(err),
//         };

//         let token_source = AccessTokenResponseSuccessfulBodySource::AuthorizationCodeGrant;

//         let user_info = match self
//             .provider
//             .fetch_user_info(
//                 token_source,
//                 &token,
//                 &self.client_with_user_info,
//                 &self.another_client_with_user_info,
//             )
//             .await
//         {
//             Ok(x) => x,
//             Err(err) => return SigninFlowRet::FetchUserInfoError((token, err)),
//         };

//         SigninFlowRet::Ok((token, user_info))
//     }
// }

// pub enum SigninFlowRet<P>
// where
//     P: Provider + ProviderExtAuthorizationCodeGrant + ProviderExtUserInfo,
//     <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
// {
//     Ok(
//         (
//             AccessTokenResponseSuccessfulBody<<P as Provider>::Scope>,
//             <P as ProviderExtUserInfo>::Output,
//         ),
//     ),
//     FetchUserInfoError(
//         (
//             AccessTokenResponseSuccessfulBody<<P as Provider>::Scope>,
//             <P as ProviderExtUserInfo>::Error,
//         ),
//     ),
//     FlowHandleCallbackError(FlowHandleCallbackError),
// }
