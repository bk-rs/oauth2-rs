pub mod signin_flow;
pub mod signin_flow_with_dyn;

pub use signin_flow::SigninFlow;
pub use signin_flow_with_dyn::{SigninFlowHandleCallbackRet, SigninFlowWithDyn};

//
#[cfg(feature = "with-github")]
pub use oauth2_github::{GithubProviderWithWebApplication, GithubScope, GithubUserInfoEndpoint};

#[cfg(feature = "with-google")]
pub use oauth2_google::{
    GoogleProviderForWebServerApps, GoogleProviderForWebServerAppsAccessType, GoogleScope,
    GoogleUserInfoEndpoint,
};
