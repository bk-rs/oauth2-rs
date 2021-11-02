//! https://datatracker.ietf.org/doc/html/rfc6749#section-5

use std::{fmt, str};

use mime::Mime;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};
use url::Url;

use crate::types::{AccessTokenType, Scope, ScopeParameter};

pub const CONTENT_TYPE: Mime = mime::APPLICATION_JSON;
pub const GENERAL_ERROR_BODY_KEY_ERROR: &str = "error";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralSuccessfulBody<SCOPE>
where
    SCOPE: Scope,
    <SCOPE as str::FromStr>::Err: fmt::Display,
{
    pub access_token: String,
    pub token_type: AccessTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeParameter<SCOPE>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extensions: Option<Map<String, Value>>,
}
impl<SCOPE> GeneralSuccessfulBody<SCOPE>
where
    SCOPE: Scope,
    <SCOPE as str::FromStr>::Err: fmt::Display,
{
    pub fn new(
        access_token: String,
        token_type: AccessTokenType,
        expires_in: Option<usize>,
        refresh_token: Option<String>,
        scope: Option<ScopeParameter<SCOPE>>,
    ) -> Self {
        Self {
            access_token,
            token_type,
            expires_in,
            refresh_token,
            scope,
            _extensions: None,
        }
    }

    pub fn set_extensions(&mut self, extensions: Map<String, Value>) {
        self._extensions = Some(extensions);
    }
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        self._extensions.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralErrorBody {
    pub error: ErrorBodyError,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<Url>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extensions: Option<Map<String, Value>>,
}
impl GeneralErrorBody {
    pub fn new(
        error: ErrorBodyError,
        error_description: Option<String>,
        error_uri: Option<Url>,
    ) -> Self {
        Self {
            error,
            error_description,
            error_uri,
            _extensions: None,
        }
    }

    pub fn set_extensions(&mut self, extensions: Map<String, Value>) {
        self._extensions = Some(extensions);
    }
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        self._extensions.as_ref()
    }
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorBodyError {
    //
    //
    //
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
    InvalidRequest,
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
    InvalidClient,
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
    InvalidGrant,
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
    UnauthorizedClient,
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
    UnsupportedGrantType,
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
    InvalidScope,
    //
    //
    //
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
    #[cfg(feature = "with-authorization-code-grant")]
    UnsupportedResponseType,
    //
    //
    //
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    #[cfg(feature = "with-device-authorization-grant")]
    AuthorizationPending,
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    #[cfg(feature = "with-device-authorization-grant")]
    SlowDown,
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    #[cfg(feature = "with-device-authorization-grant")]
    ExpiredToken,
    //
    //
    //
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    #[cfg(any(
        feature = "with-authorization-code-grant",
        feature = "with-device-authorization",
        feature = "with-implicit-grant"
    ))]
    AccessDenied,
    //
    //
    //
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1
    #[cfg(any(
        feature = "with-authorization-code-grant",
        feature = "with-implicit-grant"
    ))]
    ServerError,
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1
    #[cfg(any(
        feature = "with-authorization-code-grant",
        feature = "with-implicit-grant"
    ))]
    TemporarilyUnavailable,
    //
    //
    //
    #[serde(other)]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_de_error_body() {
        let body_str = r#"
        {
            "error": "invalid_scope"
        }
        "#;
        match serde_json::from_str::<GeneralErrorBody>(body_str) {
            Ok(body) => {
                assert_eq!(body.error, ErrorBodyError::InvalidScope);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[cfg(feature = "with-device-authorization-grant")]
    #[test]
    fn ser_de_error_body_with_device_authorization_grant() {
        let body_str = r#"
        {
            "error": "authorization_pending"
        }
        "#;
        match serde_json::from_str::<GeneralErrorBody>(body_str) {
            Ok(body) => {
                assert_eq!(body.error, ErrorBodyError::AuthorizationPending);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
