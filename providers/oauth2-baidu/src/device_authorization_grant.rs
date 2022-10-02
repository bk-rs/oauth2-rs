use std::error;

use oauth2_client::{
    oauth2_core::{
        access_token_request::BodyWithDeviceAuthorizationGrant,
        device_authorization_grant::{
            device_authorization_request::Body as DeviceAuthorizationRequestBody,
            device_authorization_response::SuccessfulBody as DeviceAuthorizationResponseSuccessfulBody,
        },
        types::ScopeParameter,
    },
    re_exports::{
        http::Method, serde_qs, thiserror, Body, ClientId, ClientSecret, Deserialize, HttpError,
        Request, SerdeQsError, Serialize, Url, UrlParseError,
    },
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{BaiduScope, DEVICE_AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct BaiduProviderWithDevice {
    client_id: ClientId,
    client_secret: ClientSecret,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl BaiduProviderWithDevice {
    pub fn new(app_key: ClientId, secret_key: ClientSecret) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id: app_key,
            client_secret: secret_key,
            token_endpoint_url: TOKEN_URL.parse()?,
            device_authorization_endpoint_url: DEVICE_AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for BaiduProviderWithDevice {
    type Scope = BaiduScope;

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
impl ProviderExtDeviceAuthorizationGrant for BaiduProviderWithDevice {
    fn device_authorization_endpoint_url(&self) -> &Url {
        &self.device_authorization_endpoint_url
    }

    fn device_authorization_request_rendering(
        &self,
        body: &DeviceAuthorizationRequestBody<BaiduScope>,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &BaiduProviderWithDevice,
            body: &DeviceAuthorizationRequestBody<BaiduScope>,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let query = BaiduDeviceAuthorizationRequestQuery {
                response_type: "device_code".to_owned(),
                client_id: body
                    .client_id
                    .to_owned()
                    .ok_or(DeviceAuthorizationRequestRenderingError::ClientIdMissing)?,
                scope: body
                    .scope
                    .to_owned()
                    .ok_or(DeviceAuthorizationRequestRenderingError::ScopeMissing)?,
            };
            let query_str = serde_qs::to_string(&query)
                .map_err(DeviceAuthorizationRequestRenderingError::SerRequestQueryFailed)?;

            let mut url = this.device_authorization_endpoint_url().to_owned();
            url.set_query(Some(query_str.as_str()));

            let request = Request::builder()
                .method(Method::GET)
                .uri(url.as_str())
                .body(vec![])
                .map_err(DeviceAuthorizationRequestRenderingError::MakeRequestFailed)?;

            Ok(request)
        }

        Some(doing(self, body))
    }

    fn device_access_token_request_rendering(
        &self,
        body: &BodyWithDeviceAuthorizationGrant,
        device_authorization_response_body: &DeviceAuthorizationResponseSuccessfulBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &BaiduProviderWithDevice,
            _body: &BodyWithDeviceAuthorizationGrant,
            device_authorization_response_body: &DeviceAuthorizationResponseSuccessfulBody,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let query = BaiduDeviceAccessTokenRequestQuery {
                grant_type: "device_token".to_owned(),
                code: device_authorization_response_body.device_code.to_owned(),
                client_id: this.client_id.to_owned(),
                client_secret: this.client_secret.to_owned(),
            };
            let query_str = serde_qs::to_string(&query)
                .map_err(DeviceAccessTokenRequestRenderingError::SerRequestQueryFailed)?;

            let mut url = this.token_endpoint_url().to_owned();
            url.set_query(Some(query_str.as_str()));

            let request = Request::builder()
                .method(Method::GET)
                .uri(url.as_str())
                .body(vec![])
                .map_err(DeviceAccessTokenRequestRenderingError::MakeRequestFailed)?;

            Ok(request)
        }

        Some(doing(self, body, device_authorization_response_body))
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct BaiduDeviceAuthorizationRequestQuery {
    pub response_type: String,
    pub client_id: String,
    pub scope: ScopeParameter<BaiduScope>,
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAuthorizationRequestRenderingError {
    #[error("ClientIdMissing")]
    ClientIdMissing,
    #[error("ScopeMissing")]
    ScopeMissing,
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
}

//
#[derive(Serialize, Deserialize)]
pub struct BaiduDeviceAccessTokenRequestQuery {
    pub grant_type: String,
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(thiserror::Error, Debug)]
pub enum DeviceAccessTokenRequestRenderingError {
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
}
