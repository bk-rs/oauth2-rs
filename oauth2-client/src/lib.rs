pub use oauth2_core;

//
pub mod re_exports;
pub mod utils;

//
pub mod provider;

pub mod authorization_code_grant;
pub mod client_credentials_grant;
pub mod device_authorization_grant;
pub mod jwt_authorization_grant;
pub mod resource_owner_password_credentials_grant;

pub mod extensions;

//
pub use provider::Provider;

pub use authorization_code_grant::provider_ext::ProviderExtAuthorizationCodeGrant;
pub use client_credentials_grant::provider_ext::ProviderExtClientCredentialsGrant;
pub use device_authorization_grant::provider_ext::ProviderExtDeviceAuthorizationGrant;
pub use jwt_authorization_grant::provider_ext::ProviderExtJwtAuthorizationGrant;
pub use resource_owner_password_credentials_grant::provider_ext::ProviderExtResourceOwnerPasswordCredentialsGrant;

pub use extensions::{Builder as ExtensionsBuilder, DefaultBuilder as DefaultExtensionsBuilder};
