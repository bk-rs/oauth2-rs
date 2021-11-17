use oauth2_client::{
    authorization_code_grant::provider_ext::ProviderExtAuthorizationCodeGrantOidcSupportType,
    re_exports::{ClientId, ClientSecret, RedirectUri, Url, UrlParseError},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{AppleScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct AppleProviderWithAppleJs {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl AppleProviderWithAppleJs {
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
impl Provider for AppleProviderWithAppleJs {
    type Scope = AppleScope;

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
impl ProviderExtAuthorizationCodeGrant for AppleProviderWithAppleJs {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn oidc_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantOidcSupportType> {
        Some(ProviderExtAuthorizationCodeGrantOidcSupportType::Force)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        None
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }

    fn authorization_request_url_modifying(&self, url: &mut Url) {
        let query_pairs: Vec<_> = url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>();
        let mut query_pairs_mut = url.query_pairs_mut();
        query_pairs_mut.clear();
        for (k, v) in query_pairs {
            match k.as_str() {
                "response_type" => {
                    query_pairs_mut.append_pair(k.as_str(), "code id_token");
                }
                "scope" => {}
                _ => {
                    query_pairs_mut.append_pair(k.as_str(), v.as_str());
                }
            }
        }
        query_pairs_mut.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use oauth2_client::{
        authorization_code_grant::{AccessTokenEndpoint, AuthorizationEndpoint},
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let provider = AppleProviderWithAppleJs::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request =
            AuthorizationEndpoint::new(&provider, vec![AppleScope::Email], "STATE".to_owned())
                .render_request()?;

        assert_eq!(request.uri(), "https://appleid.apple.com/auth/authorize?response_type=code+id_token&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&state=STATE");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = AppleProviderWithAppleJs::new(
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
                assert!(body.id_token.is_some());
            }
            Err(body) => panic!("{:?}", body),
        }

        Ok(())
    }
}
