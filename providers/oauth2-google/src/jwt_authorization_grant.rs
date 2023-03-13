use oauth2_client::{
    re_exports::{ClientId, ClientSecret, Url, UrlParseError},
    Provider, ProviderExtJwtAuthorizationGrant,
};

use crate::{GoogleScope, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct GoogleProviderForServerToServerApps {
    assertion: String,
    //
    token_endpoint_url: Url,
}
impl GoogleProviderForServerToServerApps {
    pub fn new(assertion: String) -> Result<Self, UrlParseError> {
        Ok(Self {
            assertion,
            token_endpoint_url: TOKEN_URL.parse()?,
        })
    }
}
impl Provider for GoogleProviderForServerToServerApps {
    type Scope = GoogleScope;

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
impl ProviderExtJwtAuthorizationGrant for GoogleProviderForServerToServerApps {
    fn assertion(&self) -> &str {
        &self.assertion
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use oauth2_client::{jwt_authorization_grant::AccessTokenEndpoint, re_exports::Endpoint as _};

    #[test]
    fn access_token_request() -> Result<(), Box<dyn std::error::Error>> {
        let provider = GoogleProviderForServerToServerApps::new("ASSERTION".to_owned())?;

        let endpoint = AccessTokenEndpoint::new(&provider, None);

        let request = endpoint.render_request()?;

        assert_eq!(
            request.body(),
            b"grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer&assertion=ASSERTION"
        );

        Ok(())
    }
}
