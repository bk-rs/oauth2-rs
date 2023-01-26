use oauth2_client::{
    authorization_code_grant::provider_ext::ProviderExtAuthorizationCodeGrantOidcSupportType,
    re_exports::{
        ClientId, ClientSecret, Map, RedirectUri, Serialize_enum_str, Url, UrlParseError, Value,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{ZohoScope, AUTHORIZATION_URL, TOKEN_URL};

//
//
//
#[derive(Debug, Clone)]
pub struct ZohoProviderForWebServerApps {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    pub access_type: Option<ZohoProviderForWebServerAppsAccessType>,
    pub prompt: Option<String>,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}

#[derive(Serialize_enum_str, Debug, Clone)]
pub enum ZohoProviderForWebServerAppsAccessType {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "offline")]
    Offline,
}

impl ZohoProviderForWebServerApps {
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
            prompt: None,
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

impl Provider for ZohoProviderForWebServerApps {
    type Scope = ZohoScope;

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
impl ProviderExtAuthorizationCodeGrant for ZohoProviderForWebServerApps {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn oidc_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantOidcSupportType> {
        Some(ProviderExtAuthorizationCodeGrantOidcSupportType::Yes)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![ZohoScope::Email, ZohoScope::AaaServerProfileRead])
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
        if let Some(prompt) = &self.prompt {
            map.insert("prompt".to_owned(), Value::String(prompt.to_owned()));
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
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let provider = ZohoProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?
        .configure(|x| {
            x.access_type = Some(ZohoProviderForWebServerAppsAccessType::Offline);
            x.prompt = Some("consent".into());
        });

        let request = AuthorizationEndpoint::new(&provider, vec![ZohoScope::Site24x7AdminAll])
            .configure(|x| x.state = Some("STATE".to_owned()))
            .render_request()?;

        assert_eq!(request.uri(), "https://accounts.zoho.com/oauth/v2/auth?response_type=code&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=Site24x7.Admin.All&state=STATE&access_type=offline&prompt=consent");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = ZohoProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        //
        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_with_authorization_code_grant.json"
        );
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned())
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(body) => {
                let map = body.extra().unwrap();
                assert_eq!(
                    map.get("api_domain").unwrap().as_str(),
                    Some("https://www.zohoapis.com")
                );
            }
            Err(body) => panic!("{body:?}"),
        }

        //
        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_with_authorization_code_grant_and_offline.json"
        );
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned())
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(body) => {
                assert!(body.refresh_token.is_some());
            }
            Err(body) => panic!("{body:?}"),
        }

        /*
        When scopes include email or profile
        */
        //
        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_with_authorization_code_grant_and_id_token.json"
        );
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned())
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(body) => {
                assert!(body.id_token.is_some());
            }
            Err(body) => panic!("{body:?}"),
        }

        Ok(())
    }
}
