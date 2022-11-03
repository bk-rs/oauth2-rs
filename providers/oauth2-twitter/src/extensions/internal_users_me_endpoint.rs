// [Users lookup](https://developer.twitter.com/en/docs/twitter-api/users/lookup/api-reference/get-users-me)

use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request, Response,
    SerdeJsonError, Serialize, Url, UrlParseError, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://api.twitter.com/2/users/me";

//
#[derive(Debug, Clone)]
pub struct UsersMeEndpoint {
    access_token: String,
}
impl UsersMeEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for UsersMeEndpoint {
    type RenderRequestError = UsersMeEndpointError;

    type ParseResponseOutput = User;
    type ParseResponseError = UsersMeEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(UsersMeEndpointError::UrlParseFailed)?;
        url.query_pairs_mut()
            .append_pair("user.fields", "id,name,username,created_at,description");

        let request = Request::builder()
            .uri(url.as_str())
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(UsersMeEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        if response.status().is_success() {
            let body = serde_json::from_slice::<ResponseOkBody>(response.body())
                .map_err(UsersMeEndpointError::DeResponseBodyFailed)?;

            Ok(body.data)
        } else {
            let body = serde_json::from_slice::<ResponseFailBody>(response.body())
                .map_err(UsersMeEndpointError::DeResponseBodyFailed)?;

            Err(UsersMeEndpointError::ResponseBodyIsFail(body))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseOkBody {
    pub data: User,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub created_at: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseFailBody {
    pub title: String,
    pub r#type: String,
    pub status: usize,
    pub detail: String,
}

impl core::fmt::Display for ResponseFailBody {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ResponseFailBody {}

#[derive(thiserror::Error, Debug)]
pub enum UsersMeEndpointError {
    #[error("UrlParseFailed {0}")]
    UrlParseFailed(UrlParseError),
    //
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
    //
    #[error("ResponseBodyIsFail {0:?}")]
    ResponseBodyIsFail(ResponseFailBody),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_body() {
        match serde_json::from_str::<ResponseOkBody>(include_str!(
            "../../tests/response_body_json_files/users_me.json"
        )) {
            Ok(body) => {
                assert_eq!(body.data.name, "HAWE");
            }
            Err(err) => panic!("{}", err),
        }

        match serde_json::from_str::<ResponseFailBody>(include_str!(
            "../../tests/response_body_json_files/users_me_fail.json"
        )) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }
    }
}
