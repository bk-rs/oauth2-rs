use crate::{
    oauth2_core::types::Scope, ProviderExtAuthorizationCodeGrant,
    ProviderExtDeviceAuthorizationGrant,
};

//
#[derive(Clone)]
pub enum GrantInfo<'a, SCOPE>
where
    SCOPE: Scope,
{
    AuthorizationCodeGrant(AuthorizationCodeGrantInfo<'a, SCOPE>),
    DeviceAuthorizationGrant(DeviceAuthorizationGrantInfo<'a, SCOPE>),
}

#[derive(Clone)]
pub struct AuthorizationCodeGrantInfo<'a, SCOPE>
where
    SCOPE: Scope,
{
    pub provider: &'a (dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync),
    pub authorization_request_scopes: Option<&'a Vec<SCOPE>>,
}

#[derive(Clone)]
pub struct DeviceAuthorizationGrantInfo<'a, SCOPE>
where
    SCOPE: Scope,
{
    pub provider: &'a (dyn ProviderExtDeviceAuthorizationGrant<Scope = SCOPE> + Send + Sync),
    pub authorization_request_scopes: Option<&'a Vec<SCOPE>>,
}
