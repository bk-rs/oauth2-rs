use std::{error, fmt};

pub use crate::device_authorization_grant::device_access_token_endpoint::DeviceAccessTokenEndpointRetryReason;
use dyn_clone::{clone_trait_object, DynClone};
pub use oauth2_core::{
    access_token_request::BodyWithDeviceAuthorizationGrant,
    device_authorization_grant::{
        device_authorization_request::Body as DeviceAuthorizationRequestBody,
        device_authorization_response::{
            ErrorBody as DeviceAuthorizationResponseErrorBody,
            SuccessfulBody as DeviceAuthorizationResponseSuccessfulBody,
        },
    },
    re_exports::{AccessTokenResponseErrorBody, AccessTokenResponseSuccessfulBody},
};

use crate::{
    re_exports::{Body, ClientId, ClientSecret, Map, Request, Response, Scope, Url, Value},
    Provider,
};

//
pub trait ProviderExtDeviceAuthorizationGrant: Provider + DynClone {
    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        None
    }

    fn device_authorization_endpoint_url(&self) -> &Url;

    fn device_authorization_request_body_extra(&self) -> Option<Map<String, Value>> {
        None
    }

    fn device_authorization_request_rendering(
        &self,
        _body: &DeviceAuthorizationRequestBody<<Self as Provider>::Scope>,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        None
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

    fn device_access_token_request_body_extra(
        &self,
        _body: &BodyWithDeviceAuthorizationGrant,
        _device_authorization_response_body: &DeviceAuthorizationResponseSuccessfulBody,
    ) -> Option<Map<String, Value>> {
        None
    }

    fn device_access_token_request_rendering(
        &self,
        _body: &BodyWithDeviceAuthorizationGrant,
        _device_authorization_response_body: &DeviceAuthorizationResponseSuccessfulBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        None
    }

    #[allow(clippy::type_complexity)]
    fn device_access_token_response_parsing(
        &self,
        _response: &Response<Body>,
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
        None
    }
}

clone_trait_object!(<SCOPE> ProviderExtDeviceAuthorizationGrant<Scope = SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> fmt::Debug for dyn ProviderExtDeviceAuthorizationGrant<Scope = SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProviderExtDeviceAuthorizationGrant")
            .field("client_id", &self.client_id())
            .field("token_endpoint_url", &self.token_endpoint_url().as_str())
            .field("scopes_default", &self.scopes_default())
            .field(
                "device_authorization_endpoint_url",
                &self.device_authorization_endpoint_url().as_str(),
            )
            .finish()
    }
}

//
//
//
#[derive(Debug, Clone)]
pub struct ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
{
    inner: P,
}

impl<P> ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant,
{
    pub fn new(provider: P) -> Self {
        Self { inner: provider }
    }
}

impl<P> Provider for ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant + Clone,
{
    type Scope = String;

    fn client_id(&self) -> Option<&ClientId> {
        self.inner.client_id()
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        self.inner.client_secret()
    }

    fn token_endpoint_url(&self) -> &Url {
        self.inner.token_endpoint_url()
    }

    fn extra(&self) -> Option<Map<String, Value>> {
        self.inner.extra()
    }

    // Note
}

impl<P> ProviderExtDeviceAuthorizationGrant
    for ProviderExtDeviceAuthorizationGrantStringScopeWrapper<P>
where
    P: ProviderExtDeviceAuthorizationGrant + Clone,
{
    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        self.inner
            .scopes_default()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
    }

    fn device_authorization_endpoint_url(&self) -> &Url {
        self.inner.device_authorization_endpoint_url()
    }

    fn device_authorization_request_body_extra(&self) -> Option<Map<String, Value>> {
        self.inner.device_authorization_request_body_extra()
    }

    fn device_authorization_request_rendering(
        &self,
        body: &DeviceAuthorizationRequestBody<<Self as Provider>::Scope>,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        let body =
            match DeviceAuthorizationRequestBody::<<P as Provider>::Scope>::try_from_t_with_string(
                body,
            ) {
                Ok(x) => x,
                Err(err) => return Some(Err(Box::new(err))),
            };

        self.inner.device_authorization_request_rendering(&body)
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
        self.inner.device_authorization_response_parsing(response)
    }

    fn device_access_token_request_body_extra(
        &self,
        body: &BodyWithDeviceAuthorizationGrant,
        device_authorization_response_body: &DeviceAuthorizationResponseSuccessfulBody,
    ) -> Option<Map<String, Value>> {
        self.inner
            .device_access_token_request_body_extra(body, device_authorization_response_body)
    }

    fn device_access_token_request_rendering(
        &self,
        body: &BodyWithDeviceAuthorizationGrant,
        device_authorization_response_body: &DeviceAuthorizationResponseSuccessfulBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        self.inner
            .device_access_token_request_rendering(body, device_authorization_response_body)
    }

    #[allow(clippy::type_complexity)]
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
        self.inner
            .device_access_token_response_parsing(response)
            .map(|x| {
                x.map(|y| {
                    y.map(|z| {
                        z.map(|a| {
                            AccessTokenResponseSuccessfulBody::<<Self as Provider>::Scope>::from(&a)
                        })
                    })
                })
            })
    }

    // Note
}
