use std::{cmp::max, time::Duration};

use http_api_client_endpoint::{
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
        device_authorization_response::{
            SuccessfulBody as DA_RES_SuccessfulBody, INTERVAL_DEFAULT,
        },
    },
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Error as HttpError,
    },
    serde::{de::DeserializeOwned, Serialize},
    types::Scope,
};
use serde_json::{Error as SerdeJsonError, Map, Value};
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

use crate::ProviderExtDeviceAuthorizationGrant;

//
#[derive(Clone)]
pub struct DeviceAccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    provider: &'a (dyn ProviderExtDeviceAuthorizationGrant<Scope = SCOPE> + Send + Sync),
    device_authorization_response_successful_body: DA_RES_SuccessfulBody,
    interval: Duration,
}
impl<'a, SCOPE> DeviceAccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        provider: &'a (dyn ProviderExtDeviceAuthorizationGrant<Scope = SCOPE> + Send + Sync),
        device_authorization_response_successful_body: DA_RES_SuccessfulBody,
    ) -> Self {
        let interval = max(
            device_authorization_response_successful_body.interval(),
            Duration::from_secs(INTERVAL_DEFAULT as u64),
        );
        Self {
            provider,
            device_authorization_response_successful_body,
            interval,
        }
    }
}

impl<'a, SCOPE> RetryableEndpoint for DeviceAccessTokenEndpoint<'a, SCOPE>
where
    SCOPE: Scope + Serialize + DeserializeOwned,
{
    type RetryReason = DeviceAccessTokenEndpointRetryReason;

    type RenderRequestError = DeviceAccessTokenEndpointError;

    type ParseResponseOutput = Result<RES_SuccessfulBody<SCOPE>, RES_ErrorBody>;
    type ParseResponseError = DeviceAccessTokenEndpointError;

    fn render_request(
        &self,
        _retry: Option<&RetryableEndpointRetry<Self::RetryReason>>,
    ) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut body = BodyWithDeviceAuthorizationGrant::new(
            self.device_authorization_response_successful_body
                .device_code
                .to_owned(),
            self.provider.client_id().cloned(),
            self.provider.client_secret().cloned(),
        );
        if let Some(extra) = self.provider.device_access_token_request_body_extra(
            &body,
            &self.device_authorization_response_successful_body,
        ) {
            body.set_extra(extra);
        }

        if let Some(request_ret) = self.provider.device_access_token_request_rendering(
            &body,
            &self.device_authorization_response_successful_body,
        ) {
            let request = request_ret
                .map_err(|err| DeviceAccessTokenEndpointError::CustomRenderingRequestFailed(err))?;

            return Ok(request);
        }

        let body = REQ_Body::<SCOPE>::DeviceAuthorizationGrant(body);

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
        if let Some(body_ret_ret) = self
            .provider
            .device_access_token_response_parsing(&response)
        {
            let body_ret = body_ret_ret
                .map_err(|err| DeviceAccessTokenEndpointError::CustomParsingResponseFailed(err))?;

            return Ok(body_ret);
        }

        if response.status().is_success() {
            let map = serde_json::from_slice::<Map<String, Value>>(response.body())
                .map_err(DeviceAccessTokenEndpointError::DeResponseBodyFailed)?;
            if !map.contains_key(GENERAL_ERROR_BODY_KEY_ERROR) {
                let body = serde_json::from_slice::<RES_SuccessfulBody<SCOPE>>(response.body())
                    .map_err(DeviceAccessTokenEndpointError::DeResponseBodyFailed)?;

                return Ok(Ok(Ok(body)));
            }
        }

        let body = serde_json::from_slice::<RES_ErrorBody>(response.body())
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

    fn max_retry_count(&self) -> usize {
        // 1800 / 5
        360
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceAccessTokenEndpointRetryReason {
    AuthorizationPending,
    SlowDown,
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAccessTokenEndpointError {
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
