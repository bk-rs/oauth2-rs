use oauth2_client::{
    re_exports::{ClientId, ClientSecret, Url, UrlParseError},
    Provider, ProviderExtDeviceAuthorizationGrant,
};

use crate::{GoogleScope, DEVICE_AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct GoogleProviderForTvAndDeviceApps {
    client_id: ClientId,
    client_secret: ClientSecret,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl GoogleProviderForTvAndDeviceApps {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            token_endpoint_url: TOKEN_URL.parse()?,
            device_authorization_endpoint_url: DEVICE_AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for GoogleProviderForTvAndDeviceApps {
    type Scope = GoogleScope;

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
impl ProviderExtDeviceAuthorizationGrant for GoogleProviderForTvAndDeviceApps {
    fn device_authorization_endpoint_url(&self) -> &Url {
        &self.device_authorization_endpoint_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use oauth2_client::{
        device_authorization_grant::{
            provider_ext::DeviceAuthorizationResponseSuccessfulBody, DeviceAccessTokenEndpoint,
        },
        re_exports::RetryableEndpoint as _,
    };

    #[test]
    fn access_token_request() -> Result<(), Box<dyn std::error::Error>> {
        let provider = GoogleProviderForTvAndDeviceApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
        )?;

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

        let request = endpoint.render_request(None)?;

        assert_eq!(request.body(), b"grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code&device_code=DEVICE_CODE&client_id=CLIENT_ID&client_secret=CLIENT_SECRET");

        Ok(())
    }
}
