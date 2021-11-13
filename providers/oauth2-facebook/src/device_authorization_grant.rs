use std::error;

use oauth2_client::{
    device_authorization_grant::provider_ext::{
        AccessTokenResponseErrorBody, AccessTokenResponseSuccessfulBody,
        BodyWithDeviceAuthorizationGrant, DeviceAccessTokenEndpointRetryReason,
        DeviceAuthorizationResponseErrorBody, DeviceAuthorizationResponseSuccessfulBody,
    },
    oauth2_core::{
        device_authorization_grant::device_authorization_response::{
            DeviceCode, UserCode, VerificationUri, VerificationUriComplete,
        },
        re_exports::AccessTokenResponseErrorBodyError,
    },
    re_exports::{
        serde_json, thiserror, Body, ClientId, ClientSecret, Deserialize, Map, Response,
        SerdeJsonError, Serialize, Url, UrlParseError, Value,
    },
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{FacebookScope, DEVICE_AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct FacebookProviderForDevices {
    client_access_token: String,
    pub redirect_uri: Option<String>,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl FacebookProviderForDevices {
    pub fn new(app_id: String, client_token: String) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_access_token: format!("{}|{}", app_id, client_token),
            redirect_uri: None,
            token_endpoint_url: TOKEN_URL.parse()?,
            device_authorization_endpoint_url: DEVICE_AUTHORIZATION_URL.parse()?,
        })
    }

    pub fn configure<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }
}
impl Provider for FacebookProviderForDevices {
    type Scope = FacebookScope;

    fn client_id(&self) -> Option<&ClientId> {
        None
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        None
    }

    fn token_endpoint_url(&self) -> &Url {
        &self.token_endpoint_url
    }
}
impl ProviderExtDeviceAuthorizationGrant for FacebookProviderForDevices {
    fn device_authorization_endpoint_url(&self) -> &Url {
        &self.device_authorization_endpoint_url
    }

    fn device_authorization_request_body_extensions(&self) -> Option<Map<String, Value>> {
        let mut map = Map::new();

        map.insert(
            "access_token".to_owned(),
            Value::String(self.client_access_token.to_owned()),
        );

        if let Some(redirect_uri) = &self.redirect_uri {
            map.insert(
                "redirect_uri".to_owned(),
                Value::String(redirect_uri.to_owned()),
            );
        }

        Some(map)
    }

    fn device_authorization_response_parsing(
        &self,
        response: &Response<Body>,
    ) -> Option<
        Result<
            Result<DeviceAuthorizationResponseSuccessfulBody, DeviceAuthorizationResponseErrorBody>,
            Box<dyn error::Error + Send + Sync + 'static>,
        >,
    > {
        fn doing(
            response: &Response<Body>,
        ) -> Result<
            Result<
                FacebookDeviceAuthorizationResponseSuccessfulBody,
                FacebookDeviceAuthorizationResponseErrorBody,
            >,
            Box<dyn error::Error + Send + Sync + 'static>,
        > {
            if response.status().is_success() {
                let map = serde_json::from_slice::<Map<String, Value>>(&response.body())
                    .map_err(DeviceAuthorizationResponseParsingError::DeResponseBodyFailed)?;
                if !map.contains_key("error") {
                    let body = serde_json::from_slice::<
                        FacebookDeviceAuthorizationResponseSuccessfulBody,
                    >(&response.body())
                    .map_err(DeviceAuthorizationResponseParsingError::DeResponseBodyFailed)?;

                    return Ok(Ok(body));
                }
            }

            let body = serde_json::from_slice::<FacebookDeviceAuthorizationResponseErrorBody>(
                &response.body(),
            )
            .map_err(DeviceAuthorizationResponseParsingError::DeResponseBodyFailed)?;
            Ok(Err(body))
        }

        match doing(response) {
            Ok(Ok(ok_body)) => Some(Ok(Ok(ok_body.into()))),
            Ok(Err(err_body)) => match DeviceAuthorizationResponseErrorBody::try_from(err_body) {
                Ok(err_body) => Some(Ok(Err(err_body))),
                Err(err) => Some(Err(err)),
            },
            Err(err) => Some(Err(err)),
        }
    }

    fn device_access_token_request_body_extensions(
        &self,
        body: &BodyWithDeviceAuthorizationGrant,
    ) -> Option<Map<String, Value>> {
        let mut map = Map::new();

        map.insert(
            "access_token".to_owned(),
            Value::String(self.client_access_token.to_owned()),
        );

        map.insert(
            "code".to_owned(),
            Value::String(body.device_code.to_owned()),
        );

        Some(map)
    }

    fn device_access_token_response_parsing(
        &self,
        response: &Response<Body>,
    ) -> Option<
        Result<
            Result<
                Result<
                    AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
                    AccessTokenResponseErrorBody,
                >,
                DeviceAccessTokenEndpointRetryReason,
            >,
            Box<dyn error::Error + Send + Sync + 'static>,
        >,
    > {
        fn doing(
            response: &Response<Body>,
        ) -> Result<
            Result<
                AccessTokenResponseSuccessfulBody<<FacebookProviderForDevices as Provider>::Scope>,
                FacebookDeviceAccessTokenResponseErrorBody,
            >,
            Box<dyn error::Error + Send + Sync + 'static>,
        > {
            if response.status().is_success() {
                let map = serde_json::from_slice::<Map<String, Value>>(&response.body())
                    .map_err(DeviceAccessTokenResponseParsingError::DeResponseBodyFailed)?;
                if !map.contains_key("error") {
                    let body = serde_json::from_slice::<
                        AccessTokenResponseSuccessfulBody<
                            <FacebookProviderForDevices as Provider>::Scope,
                        >,
                    >(&response.body())
                    .map_err(DeviceAccessTokenResponseParsingError::DeResponseBodyFailed)?;

                    return Ok(Ok(body));
                }
            }

            let body = serde_json::from_slice::<FacebookDeviceAccessTokenResponseErrorBody>(
                &response.body(),
            )
            .map_err(DeviceAuthorizationResponseParsingError::DeResponseBodyFailed)?;
            Ok(Err(body))
        }

        match doing(response) {
            Ok(Ok(ok_body)) => Some(Ok(Ok(Ok(ok_body)))),
            Ok(Err(err_body)) => match Result::<
                DeviceAccessTokenEndpointRetryReason,
                AccessTokenResponseErrorBody,
            >::try_from(err_body)
            {
                Ok(Ok(reason)) => Some(Ok(Err(reason))),
                Ok(Err(err_body)) => Some(Ok(Ok(Err(err_body)))),
                Err(err) => Some(Err(err)),
            },
            Err(err) => Some(Err(err)),
        }
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct FacebookDeviceAuthorizationResponseSuccessfulBody {
    pub code: DeviceCode,
    pub user_code: UserCode,
    pub verification_uri: VerificationUri,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_uri_complete: Option<VerificationUriComplete>,
    pub expires_in: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<usize>,
}
impl From<FacebookDeviceAuthorizationResponseSuccessfulBody>
    for DeviceAuthorizationResponseSuccessfulBody
{
    fn from(body: FacebookDeviceAuthorizationResponseSuccessfulBody) -> Self {
        Self::new(
            body.code.to_owned(),
            body.user_code.to_owned(),
            body.verification_uri.to_owned(),
            body.verification_uri_complete.to_owned(),
            body.expires_in.to_owned(),
            body.interval.to_owned(),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct FacebookDeviceAuthorizationResponseErrorBody {
    pub error: FacebookDeviceAuthorizationResponseErrorBodyError,
}
#[derive(Serialize, Deserialize)]
pub struct FacebookDeviceAuthorizationResponseErrorBodyError {
    pub message: String,
}
impl TryFrom<FacebookDeviceAuthorizationResponseErrorBody>
    for DeviceAuthorizationResponseErrorBody
{
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(body: FacebookDeviceAuthorizationResponseErrorBody) -> Result<Self, Self::Error> {
        let mut body_new = Self::new(
            AccessTokenResponseErrorBodyError::Other("".to_owned()),
            Some(body.error.message.to_owned()),
            None,
        );
        body_new.set_extensions(
            serde_json::to_value(body)
                .map(|x| x.as_object().cloned())?
                .ok_or_else(|| "unreachable".to_owned())?,
        );

        Ok(body_new)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAuthorizationResponseParsingError {
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}

//
#[derive(Serialize, Deserialize)]
pub struct FacebookDeviceAccessTokenResponseErrorBody {
    pub error: FacebookDeviceAccessTokenResponseErrorBodyError,
}
#[derive(Serialize, Deserialize)]
pub struct FacebookDeviceAccessTokenResponseErrorBodyError {
    pub message: String,
    pub error_subcode: Option<isize>,
}
impl TryFrom<FacebookDeviceAccessTokenResponseErrorBody>
    for Result<DeviceAccessTokenEndpointRetryReason, AccessTokenResponseErrorBody>
{
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(body: FacebookDeviceAccessTokenResponseErrorBody) -> Result<Self, Self::Error> {
        match body.error.error_subcode {
            Some(1349174) => Ok(Ok(
                DeviceAccessTokenEndpointRetryReason::AuthorizationPending,
            )),
            Some(1349172) => Ok(Ok(DeviceAccessTokenEndpointRetryReason::SlowDown)),
            Some(1349152) => {
                let mut body_new = AccessTokenResponseErrorBody::new(
                    AccessTokenResponseErrorBodyError::ExpiredToken,
                    Some(body.error.message.to_owned()),
                    None,
                );
                body_new.set_extensions(
                    serde_json::to_value(body)
                        .map(|x| x.as_object().cloned())?
                        .ok_or_else(|| "unreachable".to_owned())?,
                );

                Ok(Err(body_new))
            }
            _ => {
                let mut body_new = AccessTokenResponseErrorBody::new(
                    AccessTokenResponseErrorBodyError::Other("".to_owned()),
                    Some(body.error.message.to_owned()),
                    None,
                );
                body_new.set_extensions(
                    serde_json::to_value(body)
                        .map(|x| x.as_object().cloned())?
                        .ok_or_else(|| "unreachable".to_owned())?,
                );

                Ok(Err(body_new))
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAccessTokenResponseParsingError {
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, time::Duration};

    use oauth2_client::{
        device_authorization_grant::{DeviceAccessTokenEndpoint, DeviceAuthorizationEndpoint},
        re_exports::{Endpoint as _, RetryableEndpoint as _},
    };

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let provider =
            FacebookProviderForDevices::new("APP_ID".to_owned(), "CLIENT_TOKEN".to_owned())?;
        let endpoint = DeviceAuthorizationEndpoint::new(
            &provider,
            vec![FacebookScope::Email, FacebookScope::PublicProfile],
        );

        //
        let request = endpoint.render_request()?;

        assert_eq!(
            request.body(),
            b"scope=email+public_profile&access_token=APP_ID%7CCLIENT_TOKEN"
        );

        Ok(())
    }

    #[test]
    fn authorization_response() -> Result<(), Box<dyn error::Error>> {
        let provider =
            FacebookProviderForDevices::new("APP_ID".to_owned(), "CLIENT_TOKEN".to_owned())?;
        let endpoint = DeviceAuthorizationEndpoint::new(
            &provider,
            vec![FacebookScope::Email, FacebookScope::PublicProfile],
        );

        //
        let response_body =
            include_str!("../tests/response_body_json_files/device_authorization.json");
        let body_ret = endpoint
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;
        match body_ret {
            Ok(body) => {
                assert_eq!(body.device_code, "4c7c240847a4c10bf6850802c51dde1e")
            }
            Err(body) => panic!("{:?}", body),
        }

        //
        let response_body = include_str!(
            "../tests/response_body_json_files/device_authorization_err_when_no_access_token.json"
        );
        let body_ret = endpoint
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;
        match body_ret {
            Ok(body) => panic!("{:?}", body),
            Err(body) => assert_eq!(
                body.error_description,
                Some("(#190) This method must be called with a client access token".to_owned())
            ),
        }

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider =
            FacebookProviderForDevices::new("APP_ID".to_owned(), "CLIENT_TOKEN".to_owned())?;
        let endpoint = DeviceAccessTokenEndpoint::new(
            &provider,
            "DEVICE_CODE".to_owned(),
            Duration::from_secs(5),
        );

        //
        let request = endpoint.render_request(None)?;

        assert_eq!(request.body(), b"grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code&device_code=DEVICE_CODE&access_token=APP_ID%7CCLIENT_TOKEN&code=DEVICE_CODE");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider =
            FacebookProviderForDevices::new("APP_ID".to_owned(), "CLIENT_TOKEN".to_owned())?;
        let endpoint = DeviceAccessTokenEndpoint::new(
            &provider,
            "DEVICE_CODE".to_owned(),
            Duration::from_secs(5),
        );

        //
        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_with_device_authorization_grant.json"
        );
        let body_ret = endpoint.parse_response(
            Response::builder().body(response_body.as_bytes().to_vec())?,
            None,
        )?;
        match body_ret {
            Ok(Ok(body)) => {
                let map = body.extensions().unwrap();
                assert_eq!(
                    map.get("data_access_expiration_time").unwrap().as_u64(),
                    Some(1644569029)
                );
            }
            Ok(Err(body)) => panic!("{:?}", body),
            Err(reason) => panic!("{:?}", reason),
        }

        //
        let response_body = include_str!(
            "../tests/response_body_json_files/device_access_token_err_when_1349174.json"
        );
        let body_ret = endpoint.parse_response(
            Response::builder().body(response_body.as_bytes().to_vec())?,
            None,
        )?;
        match body_ret {
            Ok(Ok(body)) => panic!("{:?}", body),
            Ok(Err(body)) => panic!("{:?}", body),
            Err(reason) => assert_eq!(
                reason,
                DeviceAccessTokenEndpointRetryReason::AuthorizationPending
            ),
        }

        Ok(())
    }
}
