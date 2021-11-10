use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request, Response,
    SerdeJsonError, Serialize, MIME_APPLICATION_JSON,
};

pub const USER_INFO_URL: &str = "https://www.googleapis.com/oauth2/v3/userinfo";

//
#[derive(Debug, Clone)]
pub struct Oauth2V3UserInfoEndpoint {
    access_token: String,
}
impl<'a> Oauth2V3UserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for Oauth2V3UserInfoEndpoint {
    type RenderRequestError = Oauth2V3UserInfoEndpointError;

    type ParseResponseOutput = Oauth2V3UserInfo;
    type ParseResponseError = Oauth2V3UserInfoEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(USER_INFO_URL)
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(Oauth2V3UserInfoEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<Oauth2V3UserInfo>(&response.body())
            .map_err(Oauth2V3UserInfoEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Oauth2V3UserInfo {
    pub sub: String,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
}

#[derive(thiserror::Error, Debug)]
pub enum Oauth2V3UserInfoEndpointError {
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
        match serde_json::from_str::<Oauth2V3UserInfo>(include_str!(
            "../../tests/response_body_json_files/oauth2_v3_userinfo.json"
        )) {
            Ok(user_info) => {
                assert_eq!(user_info.sub, "110578243643543721809");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
