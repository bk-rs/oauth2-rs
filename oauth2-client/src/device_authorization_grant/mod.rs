//! [RFC 8628](https://datatracker.ietf.org/doc/html/rfc8628)

//
pub mod device_access_token_endpoint;
pub mod device_authorization_endpoint;

pub use device_access_token_endpoint::{DeviceAccessTokenEndpoint, DeviceAccessTokenEndpointError};
pub use device_authorization_endpoint::{
    DeviceAuthorizationEndpoint, DeviceAuthorizationEndpointError,
};

//
#[cfg(feature = "with-flow")]
pub mod flow;
#[cfg(feature = "with-flow")]
pub use flow::{Flow, FlowError};
