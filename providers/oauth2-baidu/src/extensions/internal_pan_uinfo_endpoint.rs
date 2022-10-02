// [获取用户信息](https://pan.baidu.com/union/doc/pksg0s9ns)

use oauth2_client::re_exports::{
    http::header::ACCEPT, serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request,
    Response, SerdeJsonError, Serialize, Url, UrlParseError, MIME_APPLICATION_JSON,
};

pub const URL: &str = "https://pan.baidu.com/rest/2.0/xpan/nas?method=uinfo";

//
#[derive(Debug, Clone)]
pub struct PanUinfoEndpoint {
    access_token: String,
}
impl PanUinfoEndpoint {
    pub fn new(access_token: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
        }
    }
}

impl Endpoint for PanUinfoEndpoint {
    type RenderRequestError = PanUinfoEndpointError;

    type ParseResponseOutput = Uinfo;
    type ParseResponseError = PanUinfoEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(PanUinfoEndpointError::UrlParseFailed)?;
        url.query_pairs_mut()
            .append_pair("access_token", &self.access_token);

        let request = Request::builder()
            .uri(url.as_str())
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(PanUinfoEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<Uinfo>(response.body())
            .map_err(PanUinfoEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Uinfo {
    pub baidu_name: String,
    pub netdisk_name: String,
    pub avatar_url: String,
    pub vip_type: usize,
    pub uk: usize,
}

#[derive(thiserror::Error, Debug)]
pub enum PanUinfoEndpointError {
    #[error("UrlParseFailed {0}")]
    UrlParseFailed(UrlParseError),
    //
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
        match serde_json::from_str::<Uinfo>(include_str!(
            "../../tests/response_body_json_files/pan_uinfo.json"
        )) {
            Ok(user_info) => {
                assert_eq!(user_info.baidu_name, "153*****036");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
