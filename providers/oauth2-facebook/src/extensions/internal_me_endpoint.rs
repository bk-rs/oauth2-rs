use oauth2_client::re_exports::{
    http::header::ACCEPT, serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Map,
    Request, Response, SerdeJsonError, Serialize, Value, MIME_APPLICATION_JSON,
};
use serde_aux::field_attributes::deserialize_number_from_string;

pub const URL: &str = "https://graph.facebook.com/v12.0/me/";

//
#[derive(Debug, Clone)]
pub struct MeEndpoint {
    access_token: String,
}
impl<'a> MeEndpoint {
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
        let fields =
            "email,id,first_name,last_name,middle_name,name,name_format,picture,short_name";
        let url = format!(
            "{}?fields={}&access_token={}",
            URL, fields, self.access_token
        );

        let request = Request::builder()
            .uri(url)
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
    pub email: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub name: Option<String>,
    pub name_format: Option<String>,
    pub picture: Option<Map<String, Value>>,
    pub short_name: Option<String>,
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
            Ok(user_info) => {
                assert_eq!(user_info.id, 2199743876856949);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
