use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request, Response,
    SerdeJsonError, Serialize, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://api.twitch.tv/helix/users";

//
#[derive(Debug, Clone)]
pub struct UsersEndpoint {
    access_token: String,
    client_id: String,
}
impl<'a> UsersEndpoint {
    pub fn new(access_token: impl AsRef<str>, client_id: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
            client_id: client_id.as_ref().to_owned(),
        }
    }
}

impl Endpoint for UsersEndpoint {
    type RenderRequestError = UsersEndpointError;

    type ParseResponseOutput = Users;
    type ParseResponseError = UsersEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(URL)
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header("Client-Id", &self.client_id)
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(UsersEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<Users>(&response.body())
            .map_err(UsersEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Users {
    pub data: Vec<User>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub login: String,
    pub email: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum UsersEndpointError {
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
    fn de_users() {
        match serde_json::from_str::<Users>(include_str!(
            "../../tests/response_body_json_files/users.json"
        )) {
            Ok(users) => {
                assert_eq!(users.data.len(), 1);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
