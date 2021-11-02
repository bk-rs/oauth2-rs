use std::{fmt, str};

use http_api_endpoint::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Error as HttpError,
    },
    Body, Endpoint, Request, Response,
};
use oauth2_core::{
    access_token_request::{
        Body as REQ_Body, BodyWithAuthorizationCodeGrant, CONTENT_TYPE as REQ_CONTENT_TYPE,
        METHOD as REQ_METHOD,
    },
    access_token_response::{CONTENT_TYPE as RES_CONTENT_TYPE, GENERAL_ERROR_BODY_KEY_ERROR},
    authorization_code_grant::{
        access_token_response::{ErrorBody as RES_ErrorBody, SuccessfulBody as RES_SuccessfulBody},
        authorization_response::Code,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::de::DeserializeOwned;
use serde_json::{Error as SerdeJsonError, Map, Value};
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

//
pub struct AccessTokenEndpoint<'a, P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    provider: &'a P,
    code: Code,
}
impl<'a, P> AccessTokenEndpoint<'a, P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    pub fn new(provider: &'a P, code: Code) -> Self {
        Self { provider, code }
    }
}

impl<'a, P> Endpoint for AccessTokenEndpoint<'a, P>
where
    P: ProviderExtAuthorizationCodeGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: DeserializeOwned,
{
    type RenderRequestError = AccessTokenEndpointError;

    type ParseResponseOutput = Result<RES_SuccessfulBody<<P as Provider>::Scope>, RES_ErrorBody>;
    type ParseResponseError = AccessTokenEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut body = BodyWithAuthorizationCodeGrant::new(
            self.code.to_owned(),
            self.provider.redirect_uri().map(|x| x.url().to_owned()),
            self.provider.client_id(),
        );
        body._extensions = self.provider.access_token_request_body_extensions();

        let body = REQ_Body::AuthorizationCodeGrant(body);

        let body_str = serde_urlencoded::to_string(body)
            .map_err(AccessTokenEndpointError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(REQ_METHOD)
            .uri(self.provider.token_endpoint_url().as_str())
            .header(CONTENT_TYPE, REQ_CONTENT_TYPE.to_string())
            .header(ACCEPT, RES_CONTENT_TYPE.to_string())
            .body(body_str.as_bytes().to_vec())
            .map_err(AccessTokenEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        if response.status().is_success() {
            let map = serde_json::from_slice::<Map<String, Value>>(&response.body())
                .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;
            if !map.contains_key(GENERAL_ERROR_BODY_KEY_ERROR) {
                let body = serde_json::from_slice::<RES_SuccessfulBody<<P as Provider>::Scope>>(
                    &response.body(),
                )
                .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;

                return Ok(Ok(body));
            }
        }

        let body = serde_json::from_slice::<RES_ErrorBody>(&response.body())
            .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;
        Ok(Err(body))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenEndpointError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
