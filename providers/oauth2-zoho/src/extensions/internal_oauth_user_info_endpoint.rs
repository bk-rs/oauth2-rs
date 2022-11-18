// [Using the Access token to make API requests](https://www.zoho.com/accounts/protocol/oauth/use-access-token.html)
// [Zoho API: Get the user that is making the request](https://stackoverflow.com/a/54449818/918930)

use oauth2_client::re_exports::{
    http::{
        header::{ACCEPT, AUTHORIZATION},
        StatusCode,
    },
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request, Response,
    SerdeJsonError, Serialize, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://accounts.zoho.com/oauth/user/info";

//
#[derive(Debug, Clone)]
pub struct OauthUserInfoEndpoint {
    access_token: String,
}
impl OauthUserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for OauthUserInfoEndpoint {
    type RenderRequestError = OauthUserInfoEndpointError;

    type ParseResponseOutput = OauthUserInfoResponseBodyOkJson;
    type ParseResponseError = OauthUserInfoEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(URL)
            // Or "Zoho-oauthtoken {}"
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(OauthUserInfoEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        #[allow(clippy::collapsible_else_if)]
        if response.status().is_success() {
            Ok(
                serde_json::from_slice::<OauthUserInfoResponseBodyOkJson>(response.body())
                    .map_err(OauthUserInfoEndpointError::DeResponseBodyOkJsonFailed)?,
            )
        } else {
            if let Ok(err_json) =
                serde_json::from_slice::<OauthUserInfoResponseBodyErrJson>(response.body())
            {
                Err(OauthUserInfoEndpointError::ResponseBodyError(
                    response.status(),
                    Ok(err_json),
                ))
            } else {
                Err(OauthUserInfoEndpointError::ResponseBodyError(
                    response.status(),
                    Err(String::from_utf8_lossy(response.body()).to_string()),
                ))
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OauthUserInfoResponseBodyOkJson {
    #[serde(rename = "First_Name")]
    pub first_name: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Last_Name")]
    pub last_name: String,
    #[serde(rename = "Display_Name")]
    pub display_name: String,
    #[serde(rename = "ZUID")]
    pub zuid: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OauthUserInfoResponseBodyErrJson {
    pub response: String,
    pub cause: String,
}

impl core::fmt::Display for OauthUserInfoResponseBodyErrJson {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for OauthUserInfoResponseBodyErrJson {}

#[derive(thiserror::Error, Debug)]
pub enum OauthUserInfoEndpointError {
    //
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("DeResponseBodyOkJsonFailed {0}")]
    DeResponseBodyOkJsonFailed(SerdeJsonError),
    //
    #[error("ResponseBodyError {0} {1:?}")]
    ResponseBodyError(StatusCode, Result<OauthUserInfoResponseBodyErrJson, String>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_body() {
        match serde_json::from_str::<OauthUserInfoResponseBodyOkJson>(include_str!(
            "../../tests/response_body_json_files/oauth_user_info.json"
        )) {
            Ok(body) => {
                assert_eq!(body.zuid, 795542386);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
