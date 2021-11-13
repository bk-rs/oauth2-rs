use std::error;

use oauth2_client::{
    device_authorization_grant::provider_ext::{
        BodyWithDeviceAuthorizationGrant, DeviceAuthorizationResponseSuccessfulBody,
    },
    oauth2_core::access_token_request::CONTENT_TYPE as DAT_REQ_CONTENT_TYPE,
    re_exports::{
        http::{header::CONTENT_TYPE, Method},
        serde_qs, thiserror, Body, ClientId, ClientSecret, Deserialize, HttpError, Request,
        SerdeQsError, Serialize, Url, UrlParseError,
    },
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{AmazonScope, DEVICE_AUTHORIZATION_URL, TOKEN_URL_NA};

#[derive(Debug, Clone)]
pub struct AmazonProviderWithDevices {
    client_id: ClientId,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl AmazonProviderWithDevices {
    pub fn new(client_id: ClientId) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            token_endpoint_url: TOKEN_URL_NA.parse()?,
            device_authorization_endpoint_url: DEVICE_AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for AmazonProviderWithDevices {
    type Scope = AmazonScope;

    fn client_id(&self) -> Option<&ClientId> {
        Some(&self.client_id)
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        None
    }

    fn token_endpoint_url(&self) -> &Url {
        &self.token_endpoint_url
    }
}
impl ProviderExtDeviceAuthorizationGrant for AmazonProviderWithDevices {
    fn device_authorization_endpoint_url(&self) -> &Url {
        &self.device_authorization_endpoint_url
    }

    fn device_access_token_request_rendering(
        &self,
        body: &BodyWithDeviceAuthorizationGrant,
        device_authorization_response_body: &DeviceAuthorizationResponseSuccessfulBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &AmazonProviderWithDevices,
            body: &BodyWithDeviceAuthorizationGrant,
            device_authorization_response_body: &DeviceAuthorizationResponseSuccessfulBody,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let body = AmazonDeviceAccessTokenRequestBody {
                grant_type: "device_code".to_owned(),
                device_code: body.device_code.to_owned(),
                user_code: device_authorization_response_body.user_code.to_owned(),
            };
            let body_str = serde_qs::to_string(&body)
                .map_err(DeviceAccessTokenRequestRenderingError::SerRequestBodyFailed)?;

            let request = Request::builder()
                .method(Method::POST)
                .uri(this.token_endpoint_url().as_str())
                .header(CONTENT_TYPE, DAT_REQ_CONTENT_TYPE.to_string())
                .body(body_str.as_bytes().to_vec())
                .map_err(DeviceAccessTokenRequestRenderingError::MakeRequestFailed)?;

            Ok(request)
        }

        Some(doing(self, body, device_authorization_response_body))
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct AmazonDeviceAccessTokenRequestBody {
    pub grant_type: String,
    pub device_code: String,
    pub user_code: String,
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAccessTokenRequestRenderingError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeQsError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use oauth2_client::{
        device_authorization_grant::DeviceAccessTokenEndpoint,
        oauth2_core::re_exports::AccessTokenResponseErrorBodyError,
        re_exports::{Response, RetryableEndpoint as _},
    };

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = AmazonProviderWithDevices::new("CLIENT_ID".to_owned())?;
        let endpoint = DeviceAccessTokenEndpoint::new(
            &provider,
            DeviceAuthorizationResponseSuccessfulBody::new(
                "DEVICE_CODE".to_owned(),
                "".to_owned(),
                "https://example.com".parse()?,
                None,
                0,
                Some(5),
            ),
        );

        //
        let response_body = include_str!(
            "../tests/response_body_json_files/device_access_token_err_when_unsupported_grant_type.json"
        );
        let body_ret = endpoint.parse_response(
            Response::builder().body(response_body.as_bytes().to_vec())?,
            None,
        )?;
        match body_ret {
            Ok(Ok(body)) => panic!("{:?}", body),
            Ok(Err(body)) => {
                assert_eq!(
                    body.error,
                    AccessTokenResponseErrorBodyError::UnsupportedGrantType
                );
            }
            Err(reason) => panic!("{:?}", reason),
        }

        Ok(())
    }
}
