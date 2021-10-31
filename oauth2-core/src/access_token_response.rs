//! https://datatracker.ietf.org/doc/html/rfc6749#section-5

use std::{fmt, str};

use mime::Mime;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use url::Url;

use crate::types::{AccessTokenType, Scope, ScopeParameter};

pub const CONTENT_TYPE: Mime = mime::APPLICATION_JSON;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralSuccessfulBody<SCOPE>
where
    SCOPE: Scope,
    <SCOPE as str::FromStr>::Err: fmt::Display,
{
    pub access_token: String,
    pub token_type: AccessTokenType,
    pub expires_in: Option<usize>,
    pub refresh_token: Option<String>,
    pub scope: Option<ScopeParameter<SCOPE>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralErrorBody {
    pub error: ErrorBodyError,
    pub error_description: Option<String>,
    pub error_uri: Option<Url>,
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
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    #[cfg(feature = "with-device-authorization-grant")]
    AuthorizationPending,
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    #[cfg(feature = "with-device-authorization-grant")]
    SlowDown,
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    #[cfg(feature = "with-device-authorization-grant")]
    AccessDenied,
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    #[cfg(feature = "with-device-authorization-grant")]
    ExpiredToken,
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
    fn de() {
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
    fn de_with_device_authorization_grant() {
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
