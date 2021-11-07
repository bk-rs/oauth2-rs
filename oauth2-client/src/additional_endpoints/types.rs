use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub uid: String,
    pub name: Option<String>,
    pub email: Option<String>,
    //
    pub raw: Map<String, Value>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AccessTokenObtainFrom {
    AuthorizationCodeGrant,
    DeviceAuthorizationGrant,
}
