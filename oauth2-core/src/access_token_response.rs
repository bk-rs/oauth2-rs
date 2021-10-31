//! https://datatracker.ietf.org/doc/html/rfc6749#section-5

use std::{fmt, str};

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use crate::types::{AccessTokenType, Scope, ScopeParameter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulBody<S>
where
    S: Scope,
    <S as str::FromStr>::Err: fmt::Display,
{
    pub access_token: String,
    pub token_type: AccessTokenType,
    pub expires_in: Option<usize>,
    pub refresh_token: Option<String>,
    pub scope: Option<ScopeParameter<S>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorBody {}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorBodyError {
    InvalidRequest,

    #[cfg(feature = "with-device-authorization-grant")]
    AuthorizationPending,

    #[serde(other)]
    Other(String),
}
