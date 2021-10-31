use std::{fmt, str};

use http_api_endpoint::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Error as HttpError,
    },
    Body, Request, Response, RetryableEndpoint, RetryableEndpointRetry,
};
use oauth2_core::{
    access_token_request::{
        Body as REQ_Body, BodyWithDeviceAuthorizationGrant, CONTENT_TYPE as REQ_CONTENT_TYPE,
        METHOD as REQ_METHOD,
    },
    access_token_response::{
        GeneralErrorBody as RES_ErrorBody, GeneralSuccessfulBody as RES_SuccessfulBody,
        CONTENT_TYPE as RES_CONTENT_TYPE,
    },
    Provider, ProviderExtDeviceAuthorizationGrant,
};
use serde::de::DeserializeOwned;
use serde_json::Error as SerdeJsonError;
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

//
pub struct DeviceAccessTokenEndpoint<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    provider: P,
    device_code: String,
}
impl<P> DeviceAccessTokenEndpoint<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    pub fn new(provider: P, device_code: impl AsRef<str>) -> Self {
        Self {
            provider,
            device_code: device_code.as_ref().to_owned(),
        }
    }
}

impl<P> RetryableEndpoint for DeviceAccessTokenEndpoint<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: DeserializeOwned,
{
    type RetryReason = String;

    const MAX_RETRY_COUNT: usize = 5;

    type RenderRequestError = DeviceAccessTokenEndpointError;

    type ParseResponseOutput = Result<RES_SuccessfulBody<<P as Provider>::Scope>, RES_ErrorBody>;
    type ParseResponseError = DeviceAccessTokenEndpointError;

    fn render_request(
        &self,
        retry: Option<&RetryableEndpointRetry<Self::RetryReason>>,
    ) -> Result<Request<Body>, Self::RenderRequestError> {
        let body = REQ_Body::DeviceAuthorizationGrant(BodyWithDeviceAuthorizationGrant {
            device_code: self.device_code.to_owned(),
            client_id: self.provider.client_id(),
        });

        let body_str = serde_urlencoded::to_string(body)
            .map_err(DeviceAccessTokenEndpointError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(REQ_METHOD)
            .uri(self.provider.device_authorization_endpoint_url().as_str())
            .header(CONTENT_TYPE, REQ_CONTENT_TYPE.to_string())
            .header(ACCEPT, RES_CONTENT_TYPE.to_string())
            .body(body_str.as_bytes().to_vec())
            .map_err(DeviceAccessTokenEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
        retry: Option<&RetryableEndpointRetry<Self::RetryReason>>,
    ) -> Result<Result<Self::ParseResponseOutput, Self::RetryReason>, Self::ParseResponseError>
    {
        if response.status().is_success() {
            match serde_json::from_slice::<RES_SuccessfulBody<<P as Provider>::Scope>>(
                &response.body(),
            ) {
                Ok(body) => return Ok(Ok(Ok(body))),
                Err(_) => {}
            }
        }

        match serde_json::from_slice::<RES_ErrorBody>(&response.body()) {
            Ok(body) => Ok(Ok(Err(body))),
            Err(err) => Err(DeviceAccessTokenEndpointError::DeResponseBodyFailed(err)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAccessTokenEndpointError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
