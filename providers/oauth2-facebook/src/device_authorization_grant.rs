use std::error;

use oauth2_client::{
    device_authorization_grant::provider_ext::{
        DeviceAuthorizationRequestBody, DeviceAuthorizationResponseErrorBody,
        DeviceAuthorizationResponseSuccessfulBody,
    },
    oauth2_core::{
        device_authorization_grant::device_authorization_response::{
            DeviceCode, UserCode, VerificationUri, VerificationUriComplete,
        },
        re_exports::AccessTokenResponseErrorBodyError,
    },
    re_exports::{
        serde_json, serde_qs, thiserror, Body, ClientId, ClientSecret, Deserialize, HttpError,
        Request, Response, SerdeJsonError, SerdeQsError, Serialize, Url, UrlParseError,
    },
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{FacebookScope, DEVICE_AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct FacebookProviderForDevices {
    client_id: ClientId,
    client_secret: ClientSecret,
    pub redirect_uri: Option<String>,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl FacebookProviderForDevices {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
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
        Some(&self.client_secret)
    }

    fn token_endpoint_url(&self) -> &Url {
        &self.token_endpoint_url
    }
}
impl ProviderExtDeviceAuthorizationGrant for FacebookProviderForDevices {
    fn device_authorization_endpoint_url(&self) -> &Url {
        &self.device_authorization_endpoint_url
    }

    fn device_authorization_request_rendering(
        &self,
        body: &DeviceAuthorizationRequestBody<<Self as Provider>::Scope>,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &FacebookProviderForDevices,
            body: &DeviceAuthorizationRequestBody<<FacebookProviderForDevices as Provider>::Scope>,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let client_id = body
                .client_id
                .to_owned()
                .ok_or_else(|| DeviceAuthorizationRequestRenderingError::ClientIdMissing)?;

            let client_secret = this.client_secret.to_owned();

            let query = FacebookDeviceAuthorizationRequestQuery {
                access_token: format!("{}|{}", client_id, client_secret),
                scope: body.scope.to_owned().map(|x| {
                    x.0.iter()
                        .map(|y| y.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                }),
                redirect_uri: this.redirect_uri.to_owned(),
            };
            let query_str = serde_qs::to_string(&query)
                .map_err(DeviceAuthorizationRequestRenderingError::SerRequestQueryFailed)?;

            let mut url = this.token_endpoint_url().to_owned();
            url.set_query(Some(query_str.as_str()));

            let request = Request::builder()
                .uri(url.as_str())
                .body(vec![])
                .map_err(DeviceAuthorizationRequestRenderingError::MakeRequestFailed)?;

            Ok(request)
        }

        Some(doing(self, body))
    }

    fn device_authorization_response_parsing(
        &self,
        _response: &Response<Body>,
    ) -> Option<
        Result<
            Result<DeviceAuthorizationResponseSuccessfulBody, DeviceAuthorizationResponseErrorBody>,
            Box<dyn error::Error + Send + Sync + 'static>,
        >,
    > {
        None
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct FacebookDeviceAuthorizationRequestQuery {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAuthorizationRequestRenderingError {
    #[error("ClientIdMissing")]
    ClientIdMissing,
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
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
