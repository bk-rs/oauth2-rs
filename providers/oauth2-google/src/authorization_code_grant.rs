use oauth2_client::{
    authorization_code_grant::provider_ext::ProviderExtAuthorizationCodeGrantOidcSupportType,
    re_exports::{
        thiserror, ClientId, ClientSecret, Map, RedirectUri, Serialize_enum_str, Url,
        UrlParseError, Value,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{GoogleScope, AUTHORIZATION_URL, TOKEN_URL};

//
//
//
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
    ) -> Result<Self, GoogleProviderForWebServerAppsNewError> {
        if !matches!(redirect_uri, RedirectUri::Url(_)) {
            return Err(GoogleProviderForWebServerAppsNewError::RedirectUriShouldBeAUrl);
        }

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

#[derive(thiserror::Error, Debug)]
pub enum GoogleProviderForWebServerAppsNewError {
    #[error("UrlParseError {0}")]
    UrlParseError(#[from] UrlParseError),
    //
    #[error("RedirectUriShouldBeAUrl")]
    RedirectUriShouldBeAUrl,
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

    fn oidc_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantOidcSupportType> {
        Some(ProviderExtAuthorizationCodeGrantOidcSupportType::Yes)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![
            GoogleScope::Profile,
            GoogleScope::Email,
            GoogleScope::Openid,
        ])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }

    fn authorization_request_query_extra(&self) -> Option<Map<String, Value>> {
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

//
//
//
#[derive(Debug, Clone)]
pub struct GoogleProviderForDesktopApps {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}

impl GoogleProviderForDesktopApps {
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

    pub fn configure<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }
}

impl Provider for GoogleProviderForDesktopApps {
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
impl ProviderExtAuthorizationCodeGrant for GoogleProviderForDesktopApps {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![
            GoogleScope::Email,
            GoogleScope::Profile,
            GoogleScope::Openid,
        ])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
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
    fn test_new() {
        match GoogleProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::Oob,
        ) {
            Ok(p) => panic!("{:?}", p),
            Err(GoogleProviderForWebServerAppsNewError::RedirectUriShouldBeAUrl) => {}
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let provider = GoogleProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?
        .configure(|x| {
            x.access_type = Some(GoogleProviderForWebServerAppsAccessType::Offline);
            x.include_granted_scopes = Some(true);
        });

        let request = AuthorizationEndpoint::new(&provider, vec![GoogleScope::Email])
            .configure(|x| x.state = Some("STATE".to_owned()))
            .render_request()?;

        assert_eq!(request.uri(), "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=email&state=STATE&access_type=offline&include_granted_scopes=true");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider = GoogleProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AccessTokenEndpoint::new(&provider, "CODE".to_owned()).render_request()?;

        assert_eq!(request.body(), b"grant_type=authorization_code&code=CODE&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&client_id=CLIENT_ID&client_secret=CLIENT_SECRET");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = GoogleProviderForWebServerApps::new(
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
