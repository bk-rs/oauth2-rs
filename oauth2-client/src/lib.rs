pub use oauth2_core;

//
pub mod re_exports;

//
pub mod provider;

#[cfg(feature = "with-authorization-code-grant")]
pub mod authorization_code_grant;
#[cfg(feature = "with-device-authorization-grant")]
pub mod device_authorization_grant;

#[cfg(feature = "with-additional-endpoints")]
pub mod additional_endpoints;

//
pub use provider::Provider;

#[cfg(feature = "with-authorization-code-grant")]
pub use authorization_code_grant::provider_ext::ProviderExtAuthorizationCodeGrant;
#[cfg(feature = "with-device-authorization-grant")]
pub use device_authorization_grant::provider_ext::ProviderExtDeviceAuthorizationGrant;
