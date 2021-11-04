//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    access_token_response::GeneralErrorBody,
    types::{Code, State},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulQuery {
    pub code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extensions: Option<Map<String, Value>>,
}
impl SuccessfulQuery {
    pub fn new(code: Code, state: Option<State>) -> Self {
        Self {
            code,
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

pub type ErrorQuery = GeneralErrorBody;

#[cfg(test)]
mod tests {
    use super::*;

    use url::Url;

    #[test]
    fn de() {
        let url_str = "https://client.example.com/cb?code=SplxlOBeZQQYbYS6WxSbIA&state=xyz";

        let url = url_str.parse::<Url>().unwrap();

        let query_str = url.query().unwrap();
        match serde_qs::from_str::<SuccessfulQuery>(query_str) {
            Ok(query) => {
                assert_eq!(query.code, "SplxlOBeZQQYbYS6WxSbIA");
                assert_eq!(query.state, Some("xyz".to_owned()));
            }
            Err(err) => panic!("{}", err),
        }
    }
}
