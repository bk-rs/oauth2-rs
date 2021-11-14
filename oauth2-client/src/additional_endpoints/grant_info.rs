use crate::{
    oauth2_core::types::Scope, ProviderExtAuthorizationCodeGrant,
    ProviderExtDeviceAuthorizationGrant,
};

#[derive(Clone)]
pub enum GrantInfo<'a, SCOPE>
where
    SCOPE: Scope,
{
    AuthorizationCodeGrant {
        provider: &'a (dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync),
        authorization_request_scopes: Option<&'a Vec<SCOPE>>,
    },
    DeviceAuthorizationGrant {
        provider: &'a (dyn ProviderExtDeviceAuthorizationGrant<Scope = SCOPE> + Send + Sync),
        authorization_request_scopes: Option<&'a Vec<SCOPE>>,
    },
}
