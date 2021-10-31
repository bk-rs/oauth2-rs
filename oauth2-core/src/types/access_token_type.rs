//! https://datatracker.ietf.org/doc/html/rfc6749#section-7.1

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum AccessTokenType {
    #[serde(rename = "bearer")]
    #[serde(alias = "Bearer")]
    Bearer,
    #[serde(rename = "mac")]
    Mac,
    #[serde(other)]
    Other(String),
}
