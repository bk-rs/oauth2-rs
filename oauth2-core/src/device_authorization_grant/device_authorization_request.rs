//! https://datatracker.ietf.org/doc/html/rfc8628#section-3.1

use http::Method;
use mime::Mime;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::types::{ClientId, Scope, ScopeFromStrError, ScopeParameter};

pub const METHOD: Method = Method::POST;
pub const CONTENT_TYPE: Mime = mime::APPLICATION_WWW_FORM_URLENCODED;
pub const RESPONSE_TYPE: &str = "device_code";

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body<SCOPE>
where
    SCOPE: Scope,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeParameter<SCOPE>>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}

impl<SCOPE> Body<SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(client_id: Option<ClientId>, scope: Option<ScopeParameter<SCOPE>>) -> Self {
        Self {
            client_id,
            scope,
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }

    pub fn try_from_t_with_string(body: &Body<String>) -> Result<Self, ScopeFromStrError> {
        let scope = if let Some(x) = &body.scope {
            Some(ScopeParameter::<SCOPE>::try_from_t_with_string(x)?)
        } else {
            None
        };

        let mut this = Self::new(body.client_id.to_owned(), scope);
        if let Some(extra) = body.extra() {
            this.set_extra(extra.to_owned());
        }
        Ok(this)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser() {
        let body = Body::new(
            Some("your_client_id".to_owned()),
            Some(vec!["email".to_owned(), "profile".to_owned()].into()),
        );
        match serde_urlencoded::to_string(body) {
            Ok(body_str) => {
                assert_eq!(body_str, "client_id=your_client_id&scope=email+profile");
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn de() {
        let body_str = r"client_id=1406020730&scope=example_scope";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(body) => {
                assert_eq!(body.client_id, Some("1406020730".to_owned()));
                assert_eq!(
                    body.scope,
                    Some(ScopeParameter(vec!["example_scope".to_owned()]))
                );
            }
            Err(err) => panic!("{}", err),
        }

        let body_str = r"client_id=1406020730";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(body) => {
                assert_eq!(body.client_id, Some("1406020730".to_owned()));
                assert_eq!(body.scope, None);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
