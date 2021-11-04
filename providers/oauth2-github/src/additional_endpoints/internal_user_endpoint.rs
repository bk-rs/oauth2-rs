use std::str;

use oauth2_client::{
    additional_endpoints::Endpoint,
    provider::{
        http::header::{ACCEPT, AUTHORIZATION},
        http_api_endpoint::MIME_APPLICATION_JSON,
        serde::{Deserialize, Serialize},
        serde_json, thiserror, Body, HttpError, Request, Response, SerdeJsonError,
    },
};

pub const USER_INFO_URL: &str = "https://api.github.com/user";

//
pub struct UserEndpoint {
    access_token: String,
}
impl<'a> UserEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for UserEndpoint {
    type RenderRequestError = UserEndpointError;

    type ParseResponseOutput = User;
    type ParseResponseError = UserEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(USER_INFO_URL)
            .header(AUTHORIZATION, format!("token {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(UserEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<User>(&response.body())
            .map_err(UserEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub login: String,
    pub id: usize,
    pub name: String,
    pub email: String,
}

#[derive(thiserror::Error, Debug)]
pub enum UserEndpointError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_user() {
        match serde_json::from_str::<User>(include_str!(
            "../../tests/response_body_json_files/user.json"
        )) {
            Ok(user) => {
                assert_eq!(user.id, 610852);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
