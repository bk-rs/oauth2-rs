use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request, Response,
    SerdeJsonError, Serialize, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://openidconnect.googleapis.com/v1/userinfo";

//
#[derive(Debug, Clone)]
pub struct OidcV1UserInfoEndpoint {
    access_token: String,
}
impl<'a> OidcV1UserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for OidcV1UserInfoEndpoint {
    type RenderRequestError = OidcV1UserInfoEndpointError;

    type ParseResponseOutput = OidcV1UserInfo;
    type ParseResponseError = OidcV1UserInfoEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(URL)
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(OidcV1UserInfoEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<OidcV1UserInfo>(&response.body())
            .map_err(OidcV1UserInfoEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OidcV1UserInfo {
    pub sub: String,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub locale: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum OidcV1UserInfoEndpointError {
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
    fn de_user_info() {
        match serde_json::from_str::<OidcV1UserInfo>(include_str!(
            "../../tests/response_body_json_files/openidconnect_v1_userinfo.json"
        )) {
            Ok(user_info) => {
                assert_eq!(user_info.sub, "110578243643543721809");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
