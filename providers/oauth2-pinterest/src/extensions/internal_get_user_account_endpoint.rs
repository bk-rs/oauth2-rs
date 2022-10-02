// https://developers.pinterest.com/docs/api/v5/#operation/user_account/get

use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request, Response,
    SerdeJsonError, Serialize, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://api.pinterest.com/v5/user_account";

//
#[derive(Debug, Clone)]
pub struct GetUserAccountEndpoint {
    access_token: String,
}
impl GetUserAccountEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for GetUserAccountEndpoint {
    type RenderRequestError = GetUserAccountEndpointError;

    type ParseResponseOutput = UserAccount;
    type ParseResponseError = GetUserAccountEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(URL)
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(GetUserAccountEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<UserAccount>(response.body())
            .map_err(GetUserAccountEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserAccount {
    pub account_type: String,
    // e.g. https://s.pinimg.com/images/user/default_600.png
    pub profile_image: String,
    pub website_url: String,
    pub username: String,
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserAccountEndpointError {
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
        match serde_json::from_str::<UserAccount>(include_str!(
            "../../tests/response_body_json_files/get_user_account.json"
        )) {
            Ok(user) => {
                assert_eq!(user.username, "xxx");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
