use std::error;

use oauth2_client::{
    authorization_code_grant::provider_ext::{
        AccessTokenResponseErrorBody, AccessTokenResponseSuccessfulBody,
    },
    oauth2_core::re_exports::AccessTokenResponseErrorBodyError,
    re_exports::{
        serde_json, thiserror, Body, ClientId, ClientSecret, Deserialize, Map, RedirectUri,
        Response, SerdeJsonError, Serialize, Url, UrlParseError, Value,
    },
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

    fn authorization_request_query_extra(&self) -> Option<Map<String, Value>> {
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

    #[allow(clippy::type_complexity)]
    fn access_token_response_parsing(
        &self,
        response: &Response<Body>,
    ) -> Option<
        Result<
            Result<
                AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
                AccessTokenResponseErrorBody,
            >,
            Box<dyn error::Error + Send + Sync + 'static>,
        >,
    > {
        fn doing(
            response: &Response<Body>,
        ) -> Result<
            Result<
                AccessTokenResponseSuccessfulBody<
                    <TwitchProviderForWebServerApps as Provider>::Scope,
                >,
                TwitchAccessTokenResponseErrorBody,
            >,
            Box<dyn error::Error + Send + Sync + 'static>,
        > {
            if response.status().is_success() {
                let map = serde_json::from_slice::<Map<String, Value>>(response.body())
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;
                if !map.contains_key("errcode") {
                    let body = serde_json::from_slice::<
                        AccessTokenResponseSuccessfulBody<
                            <TwitchProviderForWebServerApps as Provider>::Scope,
                        >,
                    >(response.body())
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;

                    return Ok(Ok(body));
                }
            }

            let body =
                serde_json::from_slice::<TwitchAccessTokenResponseErrorBody>(response.body())
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;
            Ok(Err(body))
        }

        Some(doing(response).map(|ret| ret.map(Into::into).map_err(Into::into)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TwitchAccessTokenResponseErrorBody {
    pub status: usize,
    pub message: String,
}
impl From<TwitchAccessTokenResponseErrorBody> for AccessTokenResponseErrorBody {
    fn from(body: TwitchAccessTokenResponseErrorBody) -> Self {
        let error = if body.message.to_ascii_lowercase().contains("invalid") {
            AccessTokenResponseErrorBodyError::InvalidRequest
        } else {
            AccessTokenResponseErrorBodyError::Other(body.status.to_string())
        };

        Self::new(error, Some(body.message), None)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenResponseParsingError {
    //
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use oauth2_client::{
        authorization_code_grant::{AccessTokenEndpoint, AuthorizationEndpoint},
        re_exports::{http::StatusCode, Endpoint as _},
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

        assert_eq!(request.uri(), "https://id.twitch.tv/oauth2/authorize?response_type=code&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=user%3Aread%3Aemail&state=STATE&force_verify=true");

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

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = TwitchProviderForWebServerApps::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        //
        let response_body = r#"{"status":400, "message":"Invalid authorization code"}"#;
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned()).parse_response(
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(response_body.as_bytes().to_vec())?,
        )?;

        match body_ret {
            Ok(body) => panic!("{:?}", body),
            Err(body) => {
                assert_eq!(
                    body.error,
                    AccessTokenResponseErrorBodyError::InvalidRequest
                );
                assert!(body
                    .error_description
                    .unwrap()
                    .contains("Invalid authorization code"));
            }
        }

        //
        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_with_authorization_code_grant.json"
        );
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned())
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(body) => {
                let map = body.extra().unwrap();
                assert!(map.is_empty());
            }
            Err(body) => panic!("{:?}", body),
        }

        Ok(())
    }
}
