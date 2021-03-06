use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::types::{AccessTokenType, Scope, ScopeParameter};

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulBody<SCOPE>
where
    SCOPE: Scope,
{
    pub access_token: String,
    pub token_type: AccessTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeParameter<SCOPE>>,

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
        scope: Option<ScopeParameter<SCOPE>>,
    ) -> Self {
        Self {
            access_token,
            token_type,
            expires_in,
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
}

impl<SCOPE> From<SuccessfulBody<SCOPE>> for crate::access_token_response::SuccessfulBody<SCOPE>
where
    SCOPE: Scope,
{
    fn from(body: SuccessfulBody<SCOPE>) -> Self {
        let mut this = Self::new(
            body.access_token.to_owned(),
            body.token_type.to_owned(),
            body.expires_in.to_owned(),
            None,
            body.scope.to_owned(),
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
pub type ErrorBody = crate::access_token_response::ErrorBody;
