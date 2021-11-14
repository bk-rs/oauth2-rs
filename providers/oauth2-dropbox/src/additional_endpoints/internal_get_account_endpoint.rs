use oauth2_client::re_exports::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Map, Request, Response,
    SerdeJsonError, Serialize, Value, MIME_APPLICATION_JSON,
};

// https://www.dropbox.com/developers/documentation/http/documentation#users-get_account
pub const URL: &str = "https://api.dropboxapi.com/2/users/get_account";

//
#[derive(Debug, Clone)]
pub struct GetAccountEndpoint {
    access_token: String,
    account_id: String,
}
impl GetAccountEndpoint {
    pub fn new(access_token: impl AsRef<str>, account_id: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().to_owned(),
            account_id: account_id.as_ref().to_owned(),
        }
    }
}

impl Endpoint for GetAccountEndpoint {
    type RenderRequestError = GetAccountEndpointError;

    type ParseResponseOutput = Account;
    type ParseResponseError = GetAccountEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let body = GetAccountEndpointRequestBody {
            account_id: self.account_id.to_owned(),
        };
        let body_str =
            serde_json::to_string(&body).map_err(GetAccountEndpointError::SerResponseBodyFailed)?;

        let request = Request::builder()
            .uri(URL)
            .method(Method::POST)
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(CONTENT_TYPE, MIME_APPLICATION_JSON)
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(body_str.as_bytes().to_vec())
            .map_err(GetAccountEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<Account>(response.body())
            .map_err(GetAccountEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetAccountEndpointRequestBody {
    pub account_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub account_id: String,
    pub email: Option<String>,
    //
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub _extensions: Option<Map<String, Value>>,
}

#[derive(thiserror::Error, Debug)]
pub enum GetAccountEndpointError {
    #[error("SerResponseBodyFailed {0}")]
    SerResponseBodyFailed(SerdeJsonError),
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
    fn de_account() {
        match serde_json::from_str::<Account>(include_str!(
            "../../tests/response_body_json_files/get_account.json"
        )) {
            Ok(account) => {
                assert_eq!(
                    account.account_id,
                    "dbid:AADMB_j_i3y-F9qhVBv68H3eRnzSJ3bZ9Nw"
                );
            }
            Err(err) => panic!("{}", err),
        }
    }
}
