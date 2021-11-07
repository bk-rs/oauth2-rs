use crate::{
    oauth2_core::types::Scope, ProviderExtAuthorizationCodeGrant,
    ProviderExtDeviceAuthorizationGrant,
};

#[derive(Clone)]
pub enum AccessTokenProvider<'a, SCOPE>
where
    SCOPE: Scope,
{
    AuthorizationCodeGrant(
        &'a (dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync),
    ),
    DeviceAuthorizationGrant(
        &'a (dyn ProviderExtDeviceAuthorizationGrant<Scope = SCOPE> + Send + Sync),
    ),
}
