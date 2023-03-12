use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Map, Request, Response,
    SerdeJsonError, Serialize, Value, MIME_APPLICATION_JSON,
};

// Ref https://www.linode.com/docs/api/profile/#profile-view
pub const URL: &str = "https://api.linode.com/v4/profile";

//
#[derive(Debug, Clone)]
pub struct MeEndpoint {
    access_token: String,
}
impl MeEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for MeEndpoint {
    type RenderRequestError = MeEndpointError;

    type ParseResponseOutput = User;
    type ParseResponseError = MeEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(URL)
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(MeEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<User>(response.body())
            .map_err(MeEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub uid: u64,
    pub email: String,
    pub username: String,
    //
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub _extra: Option<Map<String, Value>>,
}

#[derive(thiserror::Error, Debug)]
pub enum MeEndpointError {
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
            "../../tests/response_body_json_files/v4_profile.json"
        )) {
            Ok(user) => {
                assert_eq!(user.uid, 1234);
            }
            Err(err) => panic!("{err}"),
        }
    }
}
