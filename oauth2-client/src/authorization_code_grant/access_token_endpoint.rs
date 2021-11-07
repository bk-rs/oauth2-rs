use http_api_endpoint::{Body, Endpoint, Request, Response};
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
    serde::de::DeserializeOwned,
    types::{Code, Scope},
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
    provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
    code: Code,
}
impl<'a, SCOPE> AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        provider: &'a dyn ProviderExtAuthorizationCodeGrant<Scope = SCOPE>,
        code: Code,
    ) -> Self {
        Self { provider, code }
    }
}

impl<'a, SCOPE> Endpoint for AccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope + DeserializeOwned,
{
    type RenderRequestError = AccessTokenEndpointError;

    type ParseResponseOutput = Result<RES_SuccessfulBody<SCOPE>, RES_ErrorBody>;
    type ParseResponseError = AccessTokenEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut body = BodyWithAuthorizationCodeGrant::new(
            self.code.to_owned(),
            self.provider.redirect_uri().map(|x| x.url().to_owned()),
            self.provider.client_id().cloned(),
            self.provider.client_secret().cloned(),
        );
        if let Some(extensions) = self.provider.access_token_request_body_extensions() {
            body.set_extensions(extensions);
        }

        if let Some(request_ret) = self.provider.access_token_request_rendering(&body) {
            let request = request_ret.map_err(|err| {
                AccessTokenEndpointError::CustomRenderingRequestFailed(err.to_string())
            })?;

            return Ok(request);
        }

        //
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
        if let Some(body_ret_ret) = self.provider.access_token_response_parsing(&response) {
            let body_ret = body_ret_ret.map_err(|err| {
                AccessTokenEndpointError::CustomparsingResponseFailed(err.to_string())
            })?;

            return Ok(body_ret);
        }

        //
        if response.status().is_success() {
            let map = serde_json::from_slice::<Map<String, Value>>(&response.body())
                .map_err(AccessTokenEndpointError::DeResponseBodyFailed)?;
            if !map.contains_key(GENERAL_ERROR_BODY_KEY_ERROR) {
                let body = serde_json::from_slice::<RES_SuccessfulBody<SCOPE>>(&response.body())
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
    #[error("CustomRenderingRequestFailed {0}")]
    CustomRenderingRequestFailed(String),
    //
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    //
    #[error("CustomparsingResponseFailed {0}")]
    CustomparsingResponseFailed(String),
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
