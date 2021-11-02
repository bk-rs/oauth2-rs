use oauth2_core::{
    provider::{Url, UrlParseError},
    types::{ClientId, ClientSecret, RedirectUri},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{GithubScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct GithubProviderWithWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl GithubProviderWithWebApplication {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            token_endpoint_url: TOKEN_URL.parse()?,
            authorization_endpoint_url: AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for GithubProviderWithWebApplication {
    type Scope = GithubScope;

    fn client_id(&self) -> Option<ClientId> {
        Some(self.client_id.to_owned())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        Some(self.client_secret.to_owned())
    }

    fn token_endpoint_url(&self) -> Url {
        self.token_endpoint_url.to_owned()
    }
}
impl ProviderExtAuthorizationCodeGrant for GithubProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<RedirectUri> {
        Some(self.redirect_uri.to_owned())
    }

    fn authorization_endpoint_url(&self) -> Url {
        self.authorization_endpoint_url.to_owned()
    }
}
