use std::marker::PhantomData;

use oauth2_client::{
    oauth2_core::types::Scope,
    re_exports::{ClientId, ClientSecret, Url, UrlParseError},
    Provider, ProviderExtClientCredentialsGrant,
};

#[derive(Debug, Clone)]
pub struct DoorkeeperProviderWithClientCredentials<SCOPE>
where
    SCOPE: Scope,
{
    client_id: ClientId,
    client_secret: ClientSecret,
    //
    token_endpoint_url: Url,
    //
    phantom: PhantomData<SCOPE>,
}
impl<SCOPE> DoorkeeperProviderWithClientCredentials<SCOPE>
where
    SCOPE: Scope,
{
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        token_url: impl AsRef<str>,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            token_endpoint_url: token_url.as_ref().parse()?,
            phantom: PhantomData,
        })
    }
}
impl<SCOPE> Provider for DoorkeeperProviderWithClientCredentials<SCOPE>
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
impl<SCOPE> ProviderExtClientCredentialsGrant for DoorkeeperProviderWithClientCredentials<SCOPE> where
    SCOPE: Scope
{
}
