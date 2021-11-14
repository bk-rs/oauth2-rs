use oauth2_client::re_exports::{
    http::header::ACCEPT, serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request,
    Response, SerdeJsonError, Serialize, Value, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://api.weixin.qq.com/sns/userinfo";

//
#[derive(Debug, Clone)]
pub struct SnsUserinfoEndpoint {
    access_token: String,
    openid: String,
}
impl<'a> SnsUserinfoEndpoint {
    pub fn new(access_token: impl AsRef<str>, openid: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
            openid: openid.as_ref().to_owned(),
        }
    }
}

impl Endpoint for SnsUserinfoEndpoint {
    type RenderRequestError = SnsUserinfoEndpointError;

    type ParseResponseOutput = SnsUserinfo;
    type ParseResponseError = SnsUserinfoEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}?access_token={}&openid={}&lang=zh_CN",
            URL, self.access_token, self.openid
        );

        let request = Request::builder()
            .uri(url)
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(SnsUserinfoEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<SnsUserinfo>(response.body())
            .map_err(SnsUserinfoEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnsUserinfo {
    pub openid: String,
    pub nickname: Option<String>,
    pub sex: Option<usize>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub headimgurl: Option<String>,
    pub privilege: Option<Value>,
    pub unionid: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum SnsUserinfoEndpointError {
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
    fn de_sns_userinfo() {
        match serde_json::from_str::<SnsUserinfo>(include_str!(
            "../../tests/response_body_json_files/sns_userinfo.json"
        )) {
            Ok(user) => {
                assert_eq!(user.openid, "OPENID");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
