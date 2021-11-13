use std::error;

use oauth2_client::{
    device_authorization_grant::provider_ext::{
        BodyWithDeviceAuthorizationGrant, DeviceAuthorizationResponseErrorBody,
        DeviceAuthorizationResponseSuccessfulBody,
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
    client_id: ClientId,
    client_token: ClientSecret,
    pub redirect_uri: Option<String>,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl FacebookProviderForDevices {
    pub fn new(client_id: ClientId, client_token: ClientSecret) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_token,
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
        Some(&self.client_id)
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
            Value::String(format!("{}|{}", self.client_id, self.client_token)),
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
            Value::String(format!("{}|{}", self.client_id, self.client_token)),
        );

        map.insert(
            "code".to_owned(),
            Value::String(body.device_code.to_owned()),
        );

        Some(map)
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
