use oauth2_client::re_exports::{
    http::header::{ACCEPT, AUTHORIZATION},
    serde_json, thiserror, Body, Deserialize, Endpoint, HttpError, Request, Response,
    SerdeJsonError, Serialize, Url, UrlParseError, MIME_APPLICATION_JSON,
};

//
#[derive(Debug, Clone)]
pub struct AccountsVerifyCredentialsEndpoint {
    url: Url,
    access_token: String,
}
impl AccountsVerifyCredentialsEndpoint {
    pub fn new(
        base_url: impl AsRef<str>,
        access_token: impl AsRef<str>,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            url: Url::parse(base_url.as_ref())?.join("/api/v1/accounts/verify_credentials")?,
            access_token: access_token.as_ref().to_owned(),
        })
    }
}

impl Endpoint for AccountsVerifyCredentialsEndpoint {
    type RenderRequestError = AccountsVerifyCredentialsEndpointError;

    type ParseResponseOutput = Account;
    type ParseResponseError = AccountsVerifyCredentialsEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(self.url.as_str())
            .header(AUTHORIZATION, format!("Bearer {}", &self.access_token))
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(AccountsVerifyCredentialsEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<Account>(&response.body())
            .map_err(AccountsVerifyCredentialsEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub username: String,
    // TODO
}

#[derive(thiserror::Error, Debug)]
pub enum AccountsVerifyCredentialsEndpointError {
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
            "../../tests/response_body_json_files/accounts_verify_credentials.json"
        )) {
            Ok(account) => {
                assert_eq!(account.id, "107241446397110525");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
