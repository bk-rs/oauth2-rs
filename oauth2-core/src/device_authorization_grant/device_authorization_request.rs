//! https://datatracker.ietf.org/doc/html/rfc8628#section-3.1

use std::{fmt, str};

use http::Method;
use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{ClientId, Scope, ScopeParameter};

pub const METHOD: Method = Method::POST;
pub const CONTENT_TYPE: Mime = mime::APPLICATION_WWW_FORM_URLENCODED;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body<T>
where
    T: Scope,
    <T as str::FromStr>::Err: fmt::Display,
{
    pub client_id: ClientId,
    #[serde(default)]
    pub scope: Option<ScopeParameter<T>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser() {
        let body = Body {
            client_id: "Foo".to_owned(),
            scope: Some(vec!["email".to_owned(), "profile".to_owned()].into()),
        };
        match serde_urlencoded::to_string(&body) {
            Ok(body_str) => {
                assert_eq!(body_str, "client_id=Foo&scope=email+profile");
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn de() {
        let body_str = r"client_id=1406020730&scope=example_scope";
        match serde_urlencoded::from_str::<Body<String>>(body_str) {
            Ok(body) => {
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
                assert_eq!(body.scope, None);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
