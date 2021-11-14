use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Map, Request, Response,
    SerdeJsonError, Serialize, Value, MIME_APPLICATION_JSON,
};

// https://developer.atlassian.com/cloud/bitbucket/rest/api-group-users/#api-user-get
pub const URL: &str = "https://api.bitbucket.org/2.0/user";

//
#[derive(Debug, Clone)]
pub struct UserEndpoint {
    access_token: String,
}
impl UserEndpoint {
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
            .uri(URL)
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(UserEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<User>(response.body())
            .map_err(UserEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub account_id: String,
    pub username: String,
    pub nickname: String,
    pub links: UserLinks,
    pub email: Option<String>,
    //
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub _extensions: Option<Map<String, Value>>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserLinks {
    pub avatar: UserLink,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserLink {
    pub href: String,
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
                assert_eq!(
                    user.account_id,
                    "557058:6096354a-494e-458a-9c38-bc8ef021d382"
                );
            }
            Err(err) => panic!("{}", err),
        }
    }
}
