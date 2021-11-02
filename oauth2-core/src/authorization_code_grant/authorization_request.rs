//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1

use std::{fmt, str};

use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::Url;

use crate::types::{ClientId, Scope, ScopeParameter};

pub const METHOD: Method = Method::GET;
pub const RESPONSE_TYPE: &str = "code";
pub type State = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query<SCOPE>
where
    SCOPE: Scope,
    <SCOPE as str::FromStr>::Err: fmt::Display,
{
    pub response_type: String,
    pub client_id: ClientId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeParameter<SCOPE>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extensions: Option<Map<String, Value>>,
}
impl<SCOPE> Query<SCOPE>
where
    SCOPE: Scope,
    <SCOPE as str::FromStr>::Err: fmt::Display,
{
    pub fn new(
        client_id: ClientId,
        redirect_uri: Option<Url>,
        scope: Option<ScopeParameter<SCOPE>>,
        state: Option<State>,
    ) -> Self {
        Self {
            response_type: RESPONSE_TYPE.to_owned(),
            client_id,
            redirect_uri,
            scope,
            state,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser() {
        let query = Query::new(
            "your_client_id".to_owned(),
            Some("https://client.example.com/cb".parse().unwrap()),
            Some(vec!["email".to_owned(), "profile".to_owned()].into()),
            Some("STATE".to_owned()),
        );
        match serde_qs::to_string(&query) {
            Ok(query_str) => {
                assert_eq!(query_str, "response_type=code&client_id=your_client_id&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=email+profile&state=STATE");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
