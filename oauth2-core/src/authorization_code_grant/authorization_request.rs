//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.1

use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::types::{
    ClientId, CodeChallenge, CodeChallengeMethod, Nonce, Scope, ScopeFromStrError, ScopeParameter,
    State,
};

pub const METHOD: Method = Method::GET;
pub const RESPONSE_TYPE: &str = "code";

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query<SCOPE>
where
    SCOPE: Scope,
{
    pub response_type: String,
    pub client_id: ClientId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeParameter<SCOPE>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,

    // PKCE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge: Option<CodeChallenge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge_method: Option<CodeChallengeMethod>,

    // OIDC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Nonce>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    _extra: Option<Map<String, Value>>,
}
impl<SCOPE> Query<SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        client_id: ClientId,
        redirect_uri: Option<String>,
        scope: Option<ScopeParameter<SCOPE>>,
        state: Option<State>,
    ) -> Self {
        Self::internal_new(client_id, redirect_uri, scope, state, None, None)
    }

    fn internal_new(
        client_id: ClientId,
        redirect_uri: Option<String>,
        scope: Option<ScopeParameter<SCOPE>>,
        state: Option<State>,
        code_challenge: Option<(CodeChallenge, CodeChallengeMethod)>,
        nonce: Option<Nonce>,
    ) -> Self {
        Self {
            response_type: RESPONSE_TYPE.to_owned(),
            client_id,
            redirect_uri,
            scope,
            state,
            code_challenge: code_challenge.to_owned().map(|x| x.0),
            code_challenge_method: code_challenge.map(|x| x.1),
            nonce,
            _extra: None,
        }
    }

    pub fn set_extra(&mut self, extra: Map<String, Value>) {
        self._extra = Some(extra);
    }
    pub fn extra(&self) -> Option<&Map<String, Value>> {
        self._extra.as_ref()
    }

    pub fn try_from_t_with_string(query: &Query<String>) -> Result<Self, ScopeFromStrError> {
        let scope = if let Some(x) = &query.scope {
            Some(ScopeParameter::<SCOPE>::try_from_t_with_string(x)?)
        } else {
            None
        };

        let mut code_challenge = None;
        if let Some(cc) = &query.code_challenge {
            if let Some(ccm) = &query.code_challenge_method {
                code_challenge = Some((cc.to_owned(), ccm.to_owned()));
            }
        };

        let mut this = Self::internal_new(
            query.client_id.to_owned(),
            query.redirect_uri.to_owned(),
            scope,
            query.state.to_owned(),
            code_challenge,
            query.nonce.to_owned(),
        );
        if let Some(extra) = query.extra() {
            this.set_extra(extra.to_owned());
        }
        Ok(this)
    }
}

impl<SCOPE> From<&Query<SCOPE>> for Query<String>
where
    SCOPE: Scope,
{
    fn from(query: &Query<SCOPE>) -> Self {
        let mut code_challenge = None;
        if let Some(cc) = &query.code_challenge {
            if let Some(ccm) = &query.code_challenge_method {
                code_challenge = Some((cc.to_owned(), ccm.to_owned()));
            }
        };

        let mut this = Self::internal_new(
            query.client_id.to_owned(),
            query.redirect_uri.to_owned(),
            query
                .scope
                .to_owned()
                .map(|x| ScopeParameter::<String>::from(&x)),
            query.state.to_owned(),
            code_challenge,
            query.nonce.to_owned(),
        );
        if let Some(extra) = query.extra() {
            this.set_extra(extra.to_owned());
        }
        this
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_de() {
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
            Err(err) => panic!("{err}"),
        }
    }
}
