pub mod signin_flow;
pub mod signin_flow_with_dyn;

pub mod types;

pub use signin_flow::SigninFlow;
pub use signin_flow_with_dyn::SigninFlowWithDyn;

pub use types::{SigninFlowBuildAuthorizationUrlError, SigninFlowHandleCallbackRet};

//
#[cfg(feature = "with-github")]
pub use oauth2_github::{GithubProviderWithWebApplication, GithubScope, GithubUserInfoEndpoint};

#[cfg(feature = "with-google")]
pub use oauth2_google::{
    GoogleProviderForWebServerApps, GoogleProviderForWebServerAppsAccessType, GoogleScope,
    GoogleUserInfoEndpoint,
};
