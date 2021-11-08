use oauth2_client::{
    re_exports::{ClientId, ClientSecret, Map, RedirectUri, Url, UrlParseError, Value},
    Provider, ProviderExtAuthorizationCodeGrant,
};
use oauth2_doorkeeper::DoorkeeperProviderWithAuthorizationCodeFlow;

use crate::{authorization_url, token_url, MastodonScope};

#[derive(Debug, Clone)]
pub struct MastodonProviderForEndUsers {
    inner: DoorkeeperProviderWithAuthorizationCodeFlow<MastodonScope>,
    base_url: Url,
}
impl MastodonProviderForEndUsers {
    pub fn new(
        base_url: impl AsRef<str>,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            inner: DoorkeeperProviderWithAuthorizationCodeFlow::<MastodonScope>::new(
                client_id,
                client_secret,
                redirect_uri,
                token_url(base_url.as_ref())?.as_str(),
                authorization_url(base_url.as_ref())?.as_str(),
            )?,
            base_url: base_url.as_ref().parse()?,
        })
    }
}
impl Provider for MastodonProviderForEndUsers {
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
impl ProviderExtAuthorizationCodeGrant for MastodonProviderForEndUsers {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        self.inner.redirect_uri()
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![MastodonScope::Read, MastodonScope::Write])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        self.inner.authorization_endpoint_url()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use oauth2_client::{
        authorization_code_grant::{AccessTokenEndpoint, AuthorizationEndpoint},
        re_exports::Endpoint as _,
    };

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let provider = MastodonProviderForEndUsers::new(
            "https://mastodon.social/",
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AuthorizationEndpoint::new(
            &provider,
            vec![MastodonScope::Read, MastodonScope::Write],
            "STATE".to_owned(),
        )
        .render_request()?;

        assert_eq!(request.uri(), "https://mastodon.social/oauth/authorize?response_type=code&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=read+write&state=STATE");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider = MastodonProviderForEndUsers::new(
            "https://mastodon.social/",
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AccessTokenEndpoint::new(&provider, "CODE".to_owned()).render_request()?;

        assert_eq!(request.body(), b"grant_type=authorization_code&code=CODE&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&client_id=CLIENT_ID&client_secret=CLIENT_SECRET");

        Ok(())
    }
}
