use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Map, Request, Response,
    SerdeJsonError, Serialize, Value, MIME_APPLICATION_JSON,
};

// Ref https://docs.microsoft.com/en-us/graph/api/user-get
pub const URL: &str = "https://graph.microsoft.com/v1.0/me";

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
    pub id: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    pub mail: Option<String>,
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
            "../../tests/response_body_json_files/me.json"
        )) {
            Ok(user) => {
                assert_eq!(user.id, "f26b4f162cf4fb1f");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
