use oauth2_client::{
    re_exports::{ClientId, ClientSecret, Map, RedirectUri, Url, UrlParseError, Value},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{TwitchScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct TwitchProviderForWebServerApps {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    pub force_verify: Option<bool>,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}

impl TwitchProviderForWebServerApps {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            force_verify: None,
            token_endpoint_url: TOKEN_URL.parse()?,
            authorization_endpoint_url: AUTHORIZATION_URL.parse()?,
        })
    }

    pub fn configure<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }
}
impl Provider for TwitchProviderForWebServerApps {
    type Scope = TwitchScope;

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
impl ProviderExtAuthorizationCodeGrant for TwitchProviderForWebServerApps {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![TwitchScope::UserReadEmail])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }

    fn authorization_request_query_extensions(&self) -> Option<Map<String, Value>> {
        let mut map = Map::new();

        if let Some(force_verify) = &self.force_verify {
            if *force_verify {
                map.insert("force_verify".to_owned(), Value::String(true.to_string()));
            }
        }

        if map.is_empty() {
            None
        } else {
            Some(map)
        }
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
        let provider = TwitchProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?
        .configure(|x| {
            x.force_verify = Some(true);
        });

        let request = AuthorizationEndpoint::new(
            &provider,
            vec![TwitchScope::UserReadEmail],
            "STATE".to_owned(),
        )
        .render_request()?;

        assert_eq!(request.uri(), "https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=email&state=STATE&force_verify=true");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider = TwitchProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AccessTokenEndpoint::new(&provider, "CODE".to_owned()).render_request()?;

        assert_eq!(request.body(), b"grant_type=authorization_code&code=CODE&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&client_id=CLIENT_ID&client_secret=CLIENT_SECRET");

        Ok(())
    }
}
