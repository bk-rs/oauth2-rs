use oauth2_client::{
    provider::{
        serde_enum_str::Serialize_enum_str, ClientId, ClientSecret, Map, RedirectUri, Url,
        UrlParseError, Value,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{GoogleScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct GoogleProviderForWebServerApps {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    pub access_type: Option<GoogleProviderForWebServerAppsAccessType>,
    pub include_granted_scopes: Option<bool>,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}

#[derive(Serialize_enum_str, Debug, Clone)]
pub enum GoogleProviderForWebServerAppsAccessType {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "offline")]
    Offline,
}

impl GoogleProviderForWebServerApps {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            access_type: None,
            include_granted_scopes: None,
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
impl Provider for GoogleProviderForWebServerApps {
    type Scope = GoogleScope;

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
impl ProviderExtAuthorizationCodeGrant for GoogleProviderForWebServerApps {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }

    fn authorization_request_query_extensions(&self) -> Option<Map<String, Value>> {
        let mut map = Map::new();

        if let Some(access_type) = &self.access_type {
            map.insert(
                "access_type".to_owned(),
                Value::String(access_type.to_string()),
            );
        }
        if let Some(include_granted_scopes) = &self.include_granted_scopes {
            if *include_granted_scopes {
                map.insert(
                    "include_granted_scopes".to_owned(),
                    Value::String(true.to_string()),
                );
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
        http_api_endpoint::Endpoint as _,
    };

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let provider = GoogleProviderForWebServerApps::new(
            "APPID".to_owned(),
            "SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?
        .configure(|x| {
            x.access_type = Some(GoogleProviderForWebServerAppsAccessType::Offline);
            x.include_granted_scopes = Some(true);
        });

        let endpoint = AuthorizationEndpoint::new(
            &provider,
            vec![GoogleScope::Email],
            "ixax8kolzut108e1q5bgtm1er9xmklkn".to_owned(),
        );

        let request = endpoint.render_request()?;

        assert_eq!(request.uri(), "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id=APPID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=email&state=ixax8kolzut108e1q5bgtm1er9xmklkn&access_type=offline&include_granted_scopes=true");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider = GoogleProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let endpoint = AccessTokenEndpoint::new(&provider, "CODE".to_owned());

        let request = endpoint.render_request()?;

        assert_eq!(request.body(), b"grant_type=authorization_code&code=CODE&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&client_id=CLIENT_ID&client_secret=CLIENT_SECRET");

        Ok(())
    }
}
