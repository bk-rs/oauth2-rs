// [Get User Info](https://developers.tiktok.com/doc/tiktok-api-v2-get-user-info/)

use std::collections::HashMap;

use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request, Response,
    SerdeJsonError, Serialize, Url, UrlParseError, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://open.tiktokapis.com/v2/user/info/";

//
#[derive(Debug, Clone)]
pub struct V2UserInfoEndpoint {
    access_token: String,
}
impl V2UserInfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for V2UserInfoEndpoint {
    type RenderRequestError = V2UserInfoEndpointError;

    type ParseResponseOutput = UserObject;
    type ParseResponseError = V2UserInfoEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(V2UserInfoEndpointError::UrlParseFailed)?;
        url.query_pairs_mut()
            .append_pair("fields", "open_id,union_id,avatar_url,display_name,bio_description,profile_deep_link,is_verified,follower_count,following_count,likes_count");

        let request = Request::builder()
            .uri(url.as_str())
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(V2UserInfoEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<ResponseBody>(response.body())
            .map_err(V2UserInfoEndpointError::DeResponseBodyFailed)?;

        if body.error.code == "ok" {
            Ok(body.data.get("user").cloned().ok_or_else(|| {
                V2UserInfoEndpointError::ResponseBodyDataInvalid(body.data.to_owned())
            })?)
        } else {
            Err(V2UserInfoEndpointError::ResponseBodyIsError(body.error))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseBody {
    pub data: HashMap<String, UserObject>,
    pub error: ErrorObject,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserObject {
    pub open_id: String,
    pub union_id: Option<String>,
    pub avatar_url: Option<String>,
    pub display_name: Option<String>,
    pub bio_description: Option<String>,
    pub profile_deep_link: Option<String>,
    pub is_verified: Option<bool>,
    pub follower_count: Option<i64>,
    pub following_count: Option<i64>,
    pub likes_count: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorObject {
    pub code: String,
    pub message: String,
    pub log_id: String,
}

impl core::fmt::Display for ErrorObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ErrorObject {}

#[derive(thiserror::Error, Debug)]
pub enum V2UserInfoEndpointError {
    #[error("UrlParseFailed {0}")]
    UrlParseFailed(UrlParseError),
    //
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
    //
    #[error("ResponseBodyIsError {0:?}")]
    ResponseBodyIsError(ErrorObject),
    //
    #[error("ResponseBodyDataInvalid {0:?}")]
    ResponseBodyDataInvalid(HashMap<String, UserObject>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_body() {
        match serde_json::from_str::<ResponseBody>(include_str!(
            "../../tests/response_body_json_files/v2_user_info.json"
        )) {
            Ok(body) => {
                assert_eq!(
                    body.data
                        .get("user")
                        .map(|x| x.display_name.as_deref())
                        .unwrap(),
                    Some("122755990")
                );
            }
            Err(err) => panic!("{err}"),
        }
    }
}
