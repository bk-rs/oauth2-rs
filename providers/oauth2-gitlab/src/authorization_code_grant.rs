use oauth2_client::{
    authorization_code_grant::provider_ext::{
        ProviderExtAuthorizationCodeGrantOidcSupportType,
        ProviderExtAuthorizationCodeGrantPkceSupportType,
    },
    re_exports::{ClientId, ClientSecret, Map, RedirectUri, Url, UrlParseError, Value},
    Provider, ProviderExtAuthorizationCodeGrant,
};
use oauth2_doorkeeper::DoorkeeperProviderWithAuthorizationCodeFlow;

use crate::{authorization_url, token_url, GitlabScope};

#[derive(Debug, Clone)]
pub struct GitlabProviderForEndUsers {
    inner: DoorkeeperProviderWithAuthorizationCodeFlow<GitlabScope>,
    base_url: Url,
}
impl GitlabProviderForEndUsers {
    pub fn new(
        base_url: impl AsRef<str>,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            inner: DoorkeeperProviderWithAuthorizationCodeFlow::<GitlabScope>::new(
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
impl Provider for GitlabProviderForEndUsers {
    type Scope = GitlabScope;

    fn client_id(&self) -> Option<&ClientId> {
        self.inner.client_id()
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        self.inner.client_secret()
    }

    fn token_endpoint_url(&self) -> &Url {
        self.inner.token_endpoint_url()
    }

    fn extra(&self) -> Option<Map<String, Value>> {
        let mut map = Map::new();
        map.insert(
            "base_url".to_owned(),
            Value::String(self.base_url.to_string()),
        );
        Some(map)
    }
}
impl ProviderExtAuthorizationCodeGrant for GitlabProviderForEndUsers {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        self.inner.redirect_uri()
    }

    fn oidc_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantOidcSupportType> {
        Some(ProviderExtAuthorizationCodeGrantOidcSupportType::Yes)
    }

    fn pkce_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantPkceSupportType> {
        Some(ProviderExtAuthorizationCodeGrantPkceSupportType::Yes)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![
            GitlabScope::Openid,
            GitlabScope::Profile,
            GitlabScope::Email,
        ])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        self.inner.authorization_endpoint_url()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use oauth2_client::{
        authorization_code_grant::AccessTokenEndpoint,
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn access_token_response() -> Result<(), Box<dyn std::error::Error>> {
        let provider = GitlabProviderForEndUsers::new(
            "https://gitlab.com/",
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
            Ok(body) => {
                let map = body.extra().unwrap();
                assert!(map.get("created_at").is_some());
            }
            Err(body) => panic!("{body:?}"),
        }

        Ok(())
    }
}
