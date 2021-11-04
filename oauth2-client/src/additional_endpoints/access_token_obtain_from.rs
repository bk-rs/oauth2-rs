#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AccessTokenObtainFrom {
    AuthorizationCodeGrant,
    DeviceAuthorizationGrant,
}
