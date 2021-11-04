use oauth2_client::{
    re_exports::{ClientId, ClientSecret, RedirectUri, Url, UrlParseError},
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
impl ProviderExtAuthorizationCodeGrant for GithubProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![GithubScope::ReadUser, GithubScope::UserEmail])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use oauth2_client::authorization_code_grant::{access_token_endpoint, authorization_endpoint};

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let provider = GithubProviderWithWebApplication::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = authorization_endpoint::render_request(
            &provider,
            vec![GithubScope::UserEmail],
            "ixax8kolzut108e1q5bgtm1er9xmklkn".to_owned(),
        )?;

        assert_eq!(request.uri(), "https://github.com/login/oauth/authorize?response_type=code&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=user%3Aemail&state=ixax8kolzut108e1q5bgtm1er9xmklkn");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider = GithubProviderWithWebApplication::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = access_token_endpoint::render_request(&provider, "CODE".to_owned())?;

        assert_eq!(request.body(), b"grant_type=authorization_code&code=CODE&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&client_id=CLIENT_ID&client_secret=CLIENT_SECRET");

        Ok(())
    }
}
