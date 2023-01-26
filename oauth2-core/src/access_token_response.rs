//! https://datatracker.ietf.org/doc/html/rfc6749#section-5

use mime::Mime;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_json::{Map, Value};
use url::Url;

use crate::types::{AccessTokenType, IdToken, Scope, ScopeFromStrError, ScopeParameter};

pub const CONTENT_TYPE: Mime = mime::APPLICATION_JSON;
pub const GENERAL_ERROR_BODY_KEY_ERROR: &str = "error";

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulBody<SCOPE>
where
    SCOPE: Scope,
{
    pub access_token: String,
    // e.g. instagram {"access_token":"xxx", "user_id":0}
    #[serde(default)]
    pub token_type: AccessTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // e.g. bitbucket {"scopes": "account repository"}
    #[serde(alias = "scopes")]
    pub scope: Option<ScopeParameter<SCOPE>>,

    // OIDC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdToken>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub _extra: Option<Map<String, Value>>,
}
impl<SCOPE> SuccessfulBody<SCOPE>
where
    SCOPE: Scope,
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
            id_token: None,
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }

    pub fn try_from_t_with_string(
        body: &SuccessfulBody<String>,
    ) -> Result<Self, ScopeFromStrError> {
        let scope = if let Some(x) = &body.scope {
            Some(ScopeParameter::<SCOPE>::try_from_t_with_string(x)?)
        } else {
            None
        };

        let mut this = Self::new(
            body.access_token.to_owned(),
            body.token_type.to_owned(),
            body.expires_in.to_owned(),
            body.refresh_token.to_owned(),
            scope,
        );
        if let Some(extra) = body.extra() {
            this.set_extra(extra.to_owned());
        }
        Ok(this)
    }
}

impl<SCOPE> From<&SuccessfulBody<SCOPE>> for SuccessfulBody<String>
where
    SCOPE: Scope,
{
    fn from(body: &SuccessfulBody<SCOPE>) -> Self {
        let mut this = Self::new(
            body.access_token.to_owned(),
            body.token_type.to_owned(),
            body.expires_in.to_owned(),
            body.refresh_token.to_owned(),
            body.scope
                .to_owned()
                .map(|x| ScopeParameter::<String>::from(&x)),
        );
        if let Some(extra) = body.extra() {
            this.set_extra(extra.to_owned());
        }
        this
    }
}

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorBody {
    // e.g. twitch {"status":400, "message":"Invalid authorization code"}
    #[serde(default)]
    pub error: ErrorBodyError,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<Url>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}
impl ErrorBody {
    pub fn new(
        error: ErrorBodyError,
        error_description: Option<String>,
        error_uri: Option<Url>,
    ) -> Self {
        Self {
            error,
            error_description,
            error_uri,
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
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
    UnsupportedResponseType,
    //
    //
    //
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    AuthorizationPending,
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    SlowDown,
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    ExpiredToken,
    //
    //
    //
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1
    /// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    AccessDenied,
    //
    //
    //
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1
    ServerError,
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2.1
    /// https://datatracker.ietf.org/doc/html/rfc6749#section-4.2.2.1
    TemporarilyUnavailable,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Default for ErrorBodyError {
    fn default() -> Self {
        Self::Other("".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_de_error_body() {
        let body_str = r#"
        {
            "error": "invalid_scope"
        }
        "#;
        match serde_json::from_str::<ErrorBody>(body_str) {
            Ok(body) => {
                assert_eq!(body.error, ErrorBodyError::InvalidScope);
            }
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
mod tests_with_authorization_code_grant {
    use super::*;

    #[test]
    fn test_ser_de_error_body() {
        let body_str = r#"
        {
            "error": "authorization_pending"
        }
        "#;
        match serde_json::from_str::<ErrorBody>(body_str) {
            Ok(body) => {
                assert_eq!(body.error, ErrorBodyError::AuthorizationPending);
            }
            Err(err) => panic!("{err}"),
        }
    }
}
