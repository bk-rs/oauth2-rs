use oauth2_client::re_exports::{
    http::header::ACCEPT, serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request,
    Response, SerdeJsonError, Serialize, MIME_APPLICATION_JSON,
};

// Ref https://developer.amazon.com/docs/login-with-amazon/obtain-customer-profile.html
pub const URL: &str = "https://api.amazon.com/user/profile";

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
            .header("x-amz-access-token", &self.access_token)
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
    pub user_id: String,
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
                assert_eq!(user.user_id, "amzn1.account.AGH4ZOFGBUJ5KSAFB5DVL5KTBU3Q");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
