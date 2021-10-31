use std::{fmt, str};

use http_api_endpoint::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Error as HttpError,
    },
    Body, Endpoint, Request, Response,
};
use oauth2_core::{
    device_authorization_grant::{
        device_authorization_request::{
            Body as REQ_Body, CONTENT_TYPE as REQ_CONTENT_TYPE, METHOD as REQ_METHOD,
        },
        device_authorization_response::{
            ErrorBody as RES_ErrorBody, SuccessfulBody as RES_SuccessfulBody,
            CONTENT_TYPE as RES_CONTENT_TYPE,
        },
    },
    Provider, ProviderExtDeviceAuthorizationGrant,
};
use serde::Serialize;
use serde_json::Error as SerdeJsonError;
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

//
pub struct DeviceAuthorizationEndpoint<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    provider: P,
    scopes: Option<Vec<<P as Provider>::Scope>>,
}
impl<P> DeviceAuthorizationEndpoint<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    pub fn new(provider: P, scopes: impl Into<Option<Vec<<P as Provider>::Scope>>>) -> Self {
        Self {
            provider,
            scopes: scopes.into(),
        }
    }
}

impl<P> Endpoint for DeviceAuthorizationEndpoint<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
    <<P as Provider>::Scope as str::FromStr>::Err: fmt::Display,
    <P as Provider>::Scope: Serialize,
{
    type RenderRequestError = DeviceAuthorizationEndpointError;

    type ParseResponseOutput = Result<RES_SuccessfulBody, RES_ErrorBody>;
    type ParseResponseError = DeviceAuthorizationEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let body = REQ_Body {
            client_id: self.provider.client_id(),
            scope: self.scopes.to_owned().map(Into::into),
        };

        let body_str = serde_urlencoded::to_string(body)
            .map_err(DeviceAuthorizationEndpointError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(REQ_METHOD)
            .uri(self.provider.device_authorization_endpoint_url().as_str())
            .header(CONTENT_TYPE, REQ_CONTENT_TYPE.to_string())
            .header(ACCEPT, RES_CONTENT_TYPE.to_string())
            .body(body_str.as_bytes().to_vec())
            .map_err(DeviceAuthorizationEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        if response.status().is_success() {
            match serde_json::from_slice::<RES_SuccessfulBody>(&response.body()) {
                Ok(body) => return Ok(Ok(body)),
                Err(_) => {}
            }
        }

        match serde_json::from_slice::<RES_ErrorBody>(&response.body()) {
            Ok(body) => Ok(Err(body)),
            Err(err) => Err(DeviceAuthorizationEndpointError::DeResponseBodyFailed(err)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAuthorizationEndpointError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
