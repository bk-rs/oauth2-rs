use oauth2_client::{
    re_exports::{ClientId, ClientSecret, Map, Url, UrlParseError, Value},
    Provider, ProviderExtClientCredentialsGrant,
};
use oauth2_doorkeeper::DoorkeeperProviderWithClientCredentials;

use crate::{token_url, MastodonScope};

#[derive(Debug, Clone)]
pub struct MastodonProviderForEndApplications {
    inner: DoorkeeperProviderWithClientCredentials<MastodonScope>,
    base_url: Url,
}
impl MastodonProviderForEndApplications {
    pub fn new(
        base_url: impl AsRef<str>,
        client_id: ClientId,
        client_secret: ClientSecret,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            inner: DoorkeeperProviderWithClientCredentials::<MastodonScope>::new(
                client_id,
                client_secret,
                token_url(base_url.as_ref())?.as_str(),
            )?,
            base_url: base_url.as_ref().parse()?,
        })
    }
}
impl Provider for MastodonProviderForEndApplications {
    type Scope = MastodonScope;

    fn client_id(&self) -> Option<&ClientId> {
        self.inner.client_id()
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        self.inner.client_secret()
    }

    fn token_endpoint_url(&self) -> &Url {
        self.inner.token_endpoint_url()
    }

    fn extensions(&self) -> Option<Map<String, Value>> {
        let mut map = Map::new();
        map.insert(
            "base_url".to_owned(),
            Value::String(self.base_url.to_string()),
        );
        Some(map)
    }
}
impl ProviderExtClientCredentialsGrant for MastodonProviderForEndApplications {
    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![MastodonScope::Read, MastodonScope::Write])
    }
}
