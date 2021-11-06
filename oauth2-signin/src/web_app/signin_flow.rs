#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

use std::{error, marker::PhantomData};

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
};

#[cfg(feature = "with-github")]
use super::{GithubProviderWithWebApplication, GithubScope, GithubUserInfoEndpoint};
#[cfg(feature = "with-google")]
use super::{GoogleProviderForWebServerApps, GoogleScope, GoogleUserInfoEndpoint};

use super::SigninFlowHandleCallbackRet;

//
//
//
#[derive(Debug, Clone)]
pub enum SigninFlow<C>
where
    C: Client,
{
    #[cfg(feature = "with-github")]
    Github {
        flow: Flow<C>,
        provider:
            ProviderExtAuthorizationCodeGrantStringScopeWrapper<GithubProviderWithWebApplication>,
        scopes: Option<Vec<String>>,
        user_info_endpoint: GithubUserInfoEndpoint,
        client_with_user_info: C,
    },
    #[cfg(feature = "with-google")]
    Google {
        flow: Flow<C>,
        provider:
            ProviderExtAuthorizationCodeGrantStringScopeWrapper<GoogleProviderForWebServerApps>,
        scopes: Option<Vec<String>>,
        user_info_endpoint: GoogleUserInfoEndpoint,
        client_with_user_info: C,
    },
    _Priv(_SigninFlowPrivVariant<C>),
}

#[derive(Debug, Clone)]
pub struct _SigninFlowPrivVariant<C>(PhantomData<C>);

impl<C> SigninFlow<C>
where
    C: Client,
{
    #[cfg(feature = "with-github")]
    pub fn with_github(
        client: C,
        provider: GithubProviderWithWebApplication,
        scopes: impl Into<Option<Vec<GithubScope>>>,
        user_info_endpoint: GithubUserInfoEndpoint,
    ) -> Self
    where
        C: Clone,
    {
        Self::Github {
            flow: Flow::new(client.clone()),
            provider: ProviderExtAuthorizationCodeGrantStringScopeWrapper::new(provider),
            scopes: scopes
                .into()
                .map(|x| x.iter().map(|y| y.to_string()).collect()),
            user_info_endpoint,
            client_with_user_info: client.clone(),
        }
    }

    #[cfg(feature = "with-google")]
    pub fn with_google(
        client: C,
        provider: GoogleProviderForWebServerApps,
        scopes: impl Into<Option<Vec<GoogleScope>>>,
        user_info_endpoint: GoogleUserInfoEndpoint,
    ) -> Self
    where
        C: Clone,
    {
        Self::Google {
            flow: Flow::new(client.clone()),
            provider: ProviderExtAuthorizationCodeGrantStringScopeWrapper::new(provider),
            scopes: scopes
                .into()
                .map(|x| x.iter().map(|y| y.to_string()).collect()),
            user_info_endpoint,
            client_with_user_info: client.clone(),
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
        match self {
            #[cfg(feature = "with-github")]
            Self::Github {
                flow,
                provider,
                scopes,
                user_info_endpoint: _,
                client_with_user_info: _,
            } => flow.build_authorization_url(provider, scopes.to_owned(), state),
            #[cfg(feature = "with-google")]
            Self::Google {
                flow,
                provider,
                scopes,
                user_info_endpoint: _,
                client_with_user_info: _,
            } => flow.build_authorization_url(provider, scopes.to_owned(), state),
            Self::_Priv(_) => unreachable!(),
        }
    }

    pub fn build_authorization_url_with_custom_scopes(
        &self,
        custom_scopes: Vec<String>,
        state: impl Into<Option<State>>,
    ) -> Result<Url, FlowBuildAuthorizationUrlError> {
        match self {
            #[cfg(feature = "with-github")]
            Self::Github {
                flow,
                provider,
                scopes: _,
                user_info_endpoint: _,
                client_with_user_info: _,
            } => flow.build_authorization_url(provider, Some(custom_scopes), state),
            #[cfg(feature = "with-google")]
            Self::Google {
                flow,
                provider,
                scopes: _,
                user_info_endpoint: _,
                client_with_user_info: _,
            } => flow.build_authorization_url(provider, Some(custom_scopes), state),
            Self::_Priv(_) => unreachable!(),
        }
    }

    pub async fn handle_callback(
        &self,
        query: impl AsRef<str>,
        state: impl Into<Option<State>>,
    ) -> SigninFlowHandleCallbackRet {
        let access_token_ret = match self {
            #[cfg(feature = "with-github")]
            Self::Github {
                flow,
                provider,
                scopes: _,
                user_info_endpoint: _,
                client_with_user_info: _,
            } => flow.handle_callback(provider, query, state).await,
            #[cfg(feature = "with-google")]
            Self::Google {
                flow,
                provider,
                scopes: _,
                user_info_endpoint: _,
                client_with_user_info: _,
            } => flow.handle_callback(provider, query, state).await,
            Self::_Priv(_) => unreachable!(),
        };

        let access_token = match access_token_ret {
            Ok(x) => x,
            Err(err) => return SigninFlowHandleCallbackRet::FlowHandleCallbackError(err),
        };

        let access_token_obtain_from = AccessTokenObtainFrom::AuthorizationCodeGrant;

        let user_info_endpoint_obtain_from = match self {
            #[cfg(feature = "with-github")]
            Self::Github {
                flow: _,
                provider: _,
                scopes: _,
                user_info_endpoint,
                client_with_user_info: _,
            } => user_info_endpoint.obtain_from(access_token_obtain_from, &access_token),
            #[cfg(feature = "with-google")]
            Self::Google {
                flow: _,
                provider: _,
                scopes: _,
                user_info_endpoint,
                client_with_user_info: _,
            } => user_info_endpoint.obtain_from(access_token_obtain_from, &access_token),
            Self::_Priv(_) => unreachable!(),
        };

        match user_info_endpoint_obtain_from {
            EndpointOutputObtainFrom::None => {
                return SigninFlowHandleCallbackRet::OkButUserInfoNone(access_token);
            }
            EndpointOutputObtainFrom::Build => {
                let user_info_ret: Result<_, Box<dyn error::Error>> = match self {
                    #[cfg(feature = "with-github")]
                    Self::Github {
                        flow: _,
                        provider: _,
                        scopes: _,
                        user_info_endpoint,
                        client_with_user_info: _,
                    } => user_info_endpoint.build(access_token_obtain_from, &access_token),
                    #[cfg(feature = "with-google")]
                    Self::Google {
                        flow: _,
                        provider: _,
                        scopes: _,
                        user_info_endpoint,
                        client_with_user_info: _,
                    } => user_info_endpoint.build(access_token_obtain_from, &access_token),
                    Self::_Priv(_) => unreachable!(),
                };

                match user_info_ret {
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
                }
            }
            EndpointOutputObtainFrom::Respond => {}
        }

        let user_info_endpoint_request_ret = match self {
            #[cfg(feature = "with-github")]
            Self::Github {
                flow: _,
                provider: _,
                scopes: _,
                user_info_endpoint,
                client_with_user_info: _,
            } => user_info_endpoint.render_request(access_token_obtain_from, &access_token),
            #[cfg(feature = "with-google")]
            Self::Google {
                flow: _,
                provider: _,
                scopes: _,
                user_info_endpoint,
                client_with_user_info: _,
            } => user_info_endpoint.render_request(access_token_obtain_from, &access_token),
            Self::_Priv(_) => unreachable!(),
        };

        let user_info_endpoint_request = match user_info_endpoint_request_ret {
            Ok(x) => x,
            Err(err) => {
                return SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
                    access_token,
                    EndpointExecuteError::RenderRequestError(err),
                ));
            }
        };

        let user_info_endpoint_response_ret: Result<_, <C as Client>::RespondError> = match self {
            #[cfg(feature = "with-github")]
            Self::Github {
                flow: _,
                provider: _,
                scopes: _,
                user_info_endpoint: _,
                client_with_user_info,
            } => {
                client_with_user_info
                    .respond(user_info_endpoint_request)
                    .await
            }
            #[cfg(feature = "with-google")]
            Self::Google {
                flow: _,
                provider: _,
                scopes: _,
                user_info_endpoint: _,
                client_with_user_info,
            } => {
                client_with_user_info
                    .respond(user_info_endpoint_request)
                    .await
            }
            Self::_Priv(_) => unreachable!(),
        };

        let user_info_endpoint_response = match user_info_endpoint_response_ret {
            Ok(x) => x,
            Err(err) => {
                return SigninFlowHandleCallbackRet::OkButUserInfoObtainError((
                    access_token,
                    EndpointExecuteError::RespondFailed(err.to_string()),
                ));
            }
        };

        let user_info_ret = match self {
            #[cfg(feature = "with-github")]
            Self::Github {
                flow: _,
                provider: _,
                scopes: _,
                user_info_endpoint,
                client_with_user_info: _,
            } => UserInfoEndpoint::<String>::parse_response(
                user_info_endpoint,
                access_token_obtain_from,
                &access_token,
                user_info_endpoint_response,
            ),
            #[cfg(feature = "with-google")]
            Self::Google {
                flow: _,
                provider: _,
                scopes: _,
                user_info_endpoint,
                client_with_user_info: _,
            } => UserInfoEndpoint::<String>::parse_response(
                user_info_endpoint,
                access_token_obtain_from,
                &access_token,
                user_info_endpoint_response,
            ),
            Self::_Priv(_) => unreachable!(),
        };

        let user_info = match user_info_ret {
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
