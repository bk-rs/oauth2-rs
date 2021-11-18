//! https://datatracker.ietf.org/doc/html/rfc7636#section-4.3

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum CodeChallengeMethod {
    #[serde(rename = "S256")]
    Sha256,
    #[serde(rename = "plain")]
    Plain,
}
impl Default for CodeChallengeMethod {
    fn default() -> Self {
        Self::Sha256
    }
}
