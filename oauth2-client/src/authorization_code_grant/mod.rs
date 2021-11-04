//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.1

//
pub mod access_token_endpoint;
pub mod authorization_endpoint;
pub mod provider_ext;

pub use access_token_endpoint::{AccessTokenEndpoint, AccessTokenEndpointError};
pub use authorization_endpoint::{
    parse_redirect_uri_query, AuthorizationEndpointError, ParseRedirectUriQueryError,
};

//
#[cfg(feature = "with-flow")]
pub mod flow;
#[cfg(feature = "with-flow")]
pub use flow::{
    build_authorization_url, Flow, FlowBuildAuthorizationUrlError, FlowHandleCallbackError,
};
