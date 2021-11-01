//! https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.2

use serde::{Deserialize, Serialize};

use crate::access_token_response::GeneralErrorBody;

pub type Code = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessfulQuery {
    pub code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
