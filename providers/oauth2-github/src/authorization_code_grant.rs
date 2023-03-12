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

    use oauth2_client::{
        authorization_code_grant::{AccessTokenEndpoint, AuthorizationEndpoint},
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn authorization_request() -> Result<(), Box<dyn std::error::Error>> {
        let provider = GithubProviderWithWebApplication::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AuthorizationEndpoint::new(&provider, vec![GithubScope::UserEmail])
            .configure(|x| x.state = Some("STATE".to_owned()))
            .render_request()?;

        assert_eq!(request.uri(), "https://github.com/login/oauth/authorize?response_type=code&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=user%3Aemail&state=STATE");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn std::error::Error>> {
        let provider = GithubProviderWithWebApplication::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AccessTokenEndpoint::new(&provider, "CODE".to_owned()).render_request()?;

        assert_eq!(request.body(), b"grant_type=authorization_code&code=CODE&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&client_id=CLIENT_ID&client_secret=CLIENT_SECRET");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn std::error::Error>> {
        let provider = GithubProviderWithWebApplication::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_with_authorization_code_grant.json"
        );
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned())
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(_body) => {}
            Err(body) => panic!("{body:?}"),
        }

        Ok(())
    }
}
