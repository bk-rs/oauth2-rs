use std::{cmp::max, fmt, str, time::Duration};

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
        ErrorBodyError, CONTENT_TYPE as RES_CONTENT_TYPE, GENERAL_ERROR_BODY_KEY_ERROR,
    },
    device_authorization_grant::{
        device_access_token_response::{
            ErrorBody as RES_ErrorBody, SuccessfulBody as RES_SuccessfulBody,
        },
        device_authorization_response::{DeviceCode, INTERVAL_DEFAULT},
    },
    Provider, ProviderExtDeviceAuthorizationGrant,
};
use serde::de::DeserializeOwned;
use serde_json::{Error as SerdeJsonError, Map, Value};
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

//
pub struct DeviceAccessTokenEndpoint<'a, P>
where
    P: ProviderExtDeviceAuthorizationGrant,
{
    provider: &'a P,
    device_code: DeviceCode,
    interval: Duration,
}
impl<'a, P> DeviceAccessTokenEndpoint<'a, P>
where
    P: ProviderExtDeviceAuthorizationGrant,
{
    pub fn new(provider: &'a P, device_code: DeviceCode, interval: Duration) -> Self {
        Self {
            provider,
            device_code,
            interval: max(interval, Duration::from_secs(INTERVAL_DEFAULT as u64)),
        }
    }
}

impl<'a, P> RetryableEndpoint for DeviceAccessTokenEndpoint<'a, P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: DeserializeOwned,
{
    type RetryReason = DeviceAccessTokenEndpointRetryReason;
    // 1800 / 5
    const MAX_RETRY_COUNT: usize = 360;

    type RenderRequestError = DeviceAccessTokenEndpointError;

    type ParseResponseOutput = Result<RES_SuccessfulBody<<P as Provider>::Scope>, RES_ErrorBody>;
    type ParseResponseError = DeviceAccessTokenEndpointError;

    fn render_request(
        &self,
        _retry: Option<&RetryableEndpointRetry<Self::RetryReason>>,
    ) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut body = BodyWithDeviceAuthorizationGrant::new(
            self.device_code.to_owned(),
            self.provider.client_id(),
        );
        body._extensions = self.provider.device_access_token_request_body_extensions();

        let body = REQ_Body::DeviceAuthorizationGrant(body);

        let body_str = serde_urlencoded::to_string(body)
            .map_err(DeviceAccessTokenEndpointError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(REQ_METHOD)
            .uri(self.provider.token_endpoint_url().as_str())
            .header(CONTENT_TYPE, REQ_CONTENT_TYPE.to_string())
            .header(ACCEPT, RES_CONTENT_TYPE.to_string())
            .body(body_str.as_bytes().to_vec())
            .map_err(DeviceAccessTokenEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
        _retry: Option<&RetryableEndpointRetry<Self::RetryReason>>,
    ) -> Result<Result<Self::ParseResponseOutput, Self::RetryReason>, Self::ParseResponseError>
    {
        if response.status().is_success() {
            let map = serde_json::from_slice::<Map<String, Value>>(&response.body())
                .map_err(DeviceAccessTokenEndpointError::DeResponseBodyFailed)?;
            if !map.contains_key(GENERAL_ERROR_BODY_KEY_ERROR) {
                let body = serde_json::from_slice::<RES_SuccessfulBody<<P as Provider>::Scope>>(
                    &response.body(),
                )
                .map_err(DeviceAccessTokenEndpointError::DeResponseBodyFailed)?;

                return Ok(Ok(Ok(body)));
            }
        }

        let body = serde_json::from_slice::<RES_ErrorBody>(&response.body())
            .map_err(DeviceAccessTokenEndpointError::DeResponseBodyFailed)?;
        match body.error {
            ErrorBodyError::AuthorizationPending => {
                return Ok(Err(
                    DeviceAccessTokenEndpointRetryReason::AuthorizationPending,
                ))
            }
            ErrorBodyError::SlowDown => {
                return Ok(Err(DeviceAccessTokenEndpointRetryReason::SlowDown))
            }
            _ => {}
        }
        Ok(Ok(Err(body)))
    }

    fn next_retry_in(&self, retry: &RetryableEndpointRetry<Self::RetryReason>) -> Duration {
        match retry.reason {
            DeviceAccessTokenEndpointRetryReason::AuthorizationPending => self.interval,
            DeviceAccessTokenEndpointRetryReason::SlowDown => self.interval,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeviceAccessTokenEndpointRetryReason {
    AuthorizationPending,
    SlowDown,
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
