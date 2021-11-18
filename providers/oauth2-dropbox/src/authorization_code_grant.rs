use oauth2_client::{
    authorization_code_grant::provider_ext::ProviderExtAuthorizationCodeGrantPkceSupportType,
    re_exports::{ClientId, ClientSecret, RedirectUri, Url, UrlParseError},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{DropboxScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct DropboxProviderWithWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl DropboxProviderWithWebApplication {
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
impl Provider for DropboxProviderWithWebApplication {
    type Scope = DropboxScope;

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
impl ProviderExtAuthorizationCodeGrant for DropboxProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn pkce_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantPkceSupportType> {
        Some(ProviderExtAuthorizationCodeGrantPkceSupportType::Yes)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![
            DropboxScope::AccountInfoRead,
            DropboxScope::SharingRead,
        ])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }
}
