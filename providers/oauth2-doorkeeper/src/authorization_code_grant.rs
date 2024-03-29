use core::marker::PhantomData;

use oauth2_client::{
    oauth2_core::types::Scope,
    re_exports::{ClientId, ClientSecret, RedirectUri, Url, UrlParseError},
    Provider, ProviderExtAuthorizationCodeGrant,
};

#[derive(Debug, Clone)]
pub struct DoorkeeperProviderWithAuthorizationCodeFlow<SCOPE>
where
    SCOPE: Scope,
{
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
    //
    phantom: PhantomData<SCOPE>,
}
impl<SCOPE> DoorkeeperProviderWithAuthorizationCodeFlow<SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
        token_url: impl AsRef<str>,
        authorization_url: impl AsRef<str>,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            token_endpoint_url: token_url.as_ref().parse()?,
            authorization_endpoint_url: authorization_url.as_ref().parse()?,
            phantom: PhantomData,
        })
    }
}
impl<SCOPE> Provider for DoorkeeperProviderWithAuthorizationCodeFlow<SCOPE>
where
    SCOPE: Scope,
{
    type Scope = SCOPE;

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
impl<SCOPE> ProviderExtAuthorizationCodeGrant for DoorkeeperProviderWithAuthorizationCodeFlow<SCOPE>
where
    SCOPE: Scope,
{
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }
}
