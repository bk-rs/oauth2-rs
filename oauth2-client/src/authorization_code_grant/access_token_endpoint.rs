use http_api_client_endpoint::{Body, Endpoint, Request, Response};
use oauth2_core::{
    access_token_request::{
        Body as REQ_Body, BodyWithAuthorizationCodeGrant, CONTENT_TYPE as REQ_CONTENT_TYPE,
        METHOD as REQ_METHOD,
    },
    access_token_response::{CONTENT_TYPE as RES_CONTENT_TYPE, GENERAL_ERROR_BODY_KEY_ERROR},
    authorization_code_grant::access_token_response::{
        ErrorBody as RES_ErrorBody, SuccessfulBody as RES_SuccessfulBody,
    },
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Error as HttpError,
    },
    serde::{de::DeserializeOwned, Serialize},
    types::{Code, CodeVerifier, Scope},
};
use serde_json::{Error as SerdeJsonError, Map, Value};
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

use crate::ProviderExtAuthorizationCodeGrant;

//
//
//
#[derive(Clone)]
pub struct AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    provider: &'a (dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync),
    code: Code,
    pub code_verifier: Option<CodeVerifier>,
}
impl<'a, SCOPE> AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        provider: &'a (dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE> + Send + Sync),
        code: Code,
    ) -> Self {
        Self {
            provider,
            code,
            code_verifier: None,
        }
    }

    pub fn configure<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }

    pub fn set_code_verifier(&mut self, code_verifier: CodeVerifier) {
        self.code_verifier = Some(code_verifier);
    }
}

impl<'a, SCOPE> Endpoint for AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope + Serialize + DeserializeOwned,
{
    type RenderRequestError = AccessTokenEndpointError;

    type ParseResponseOutput = Result<RES_SuccessfulBody<SCOPE>, RES_ErrorBody>;
    type ParseResponseError = AccessTokenEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut body = BodyWithAuthorizationCodeGrant::new(
            self.code.to_owned(),
            self.provider.redirect_uri().map(|x| x.to_string()),
            self.provider.client_id().cloned(),
            self.provider.client_secret().cloned(),
        );

        if let Some(code_verifier) = &self.code_verifier {
            body.code_verifier = Some(code_verifier.to_owned());
        }

        if let Some(extra) = self.provider.access_token_request_body_extra(&body) {
            body.set_extra(extra);
        }

        if let Some(request_ret) = self.provider.access_token_request_rendering(&body) {
            let request = request_ret
                .map_err(|err| AccessTokenEndpointError::CustomRenderingRequestFailed(err))?;

            return Ok(request);
        }

        //
        let body = REQ_Body::<SCOPE>::AuthorizationCodeGrant(body);

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
        if let Some(body_ret_ret) = self.provider.access_token_response_parsing(&response) {
            let body_ret = body_ret_ret
                .map_err(|err| AccessTokenEndpointError::CustomParsingResponseFailed(err))?;

            return Ok(body_ret);
        }

        //
        if response.status().is_success() {
            let map = serde_json::from_slice::<Map<String, Value>>(response.body())
                .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;
            if !map.contains_key(GENERAL_ERROR_BODY_KEY_ERROR) {
                let body = serde_json::from_slice::<RES_SuccessfulBody<SCOPE>>(response.body())
                    .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;

                return Ok(Ok(body));
            }
        }

        let body = serde_json::from_slice::<RES_ErrorBody>(response.body())
            .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;
        Ok(Err(body))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenEndpointError {
    #[error("CustomRenderingRequestFailed {0}")]
    CustomRenderingRequestFailed(Box<dyn std::error::Error + Send + Sync>),
    //
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("CustomParsingResponseFailed {0}")]
    CustomParsingResponseFailed(Box<dyn std::error::Error + Send + Sync>),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
