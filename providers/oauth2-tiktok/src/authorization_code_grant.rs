use std::error;

use oauth2_client::{
    authorization_code_grant::provider_ext::AccessTokenRequestBody,
    oauth2_core::{
        access_token_request::GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
        re_exports::{
            AccessTokenResponseErrorBody, AccessTokenResponseErrorBodyError,
            AccessTokenResponseSuccessfulBody,
        },
        types::AccessTokenType,
    },
    re_exports::{
        http::Method, serde_json, serde_qs, thiserror, Body, ClientId, ClientSecret, HttpError,
        Map, RedirectUri, Request, Response, SerdeJsonError, SerdeQsError, Url, UrlParseError,
        Value,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::{Deserialize, Serialize};

use crate::{TiktokScope, AUTHORIZATION_URL, TOKEN_URL};

pub const KEY_OPENID: &str = "open_id";

//
#[derive(Debug, Clone)]
pub struct TiktokProviderWithWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl TiktokProviderWithWebApplication {
    pub fn new(
        client_key: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id: client_key,
            client_secret,
            redirect_uri,
            token_endpoint_url: TOKEN_URL.parse()?,
            authorization_endpoint_url: AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for TiktokProviderWithWebApplication {
    type Scope = TiktokScope;

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
impl ProviderExtAuthorizationCodeGrant for TiktokProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![TiktokScope::UserInfoBasic, TiktokScope::VideoList])
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
                "client_id" => {
                    query_pairs_mut.append_pair("client_key", v.as_str());
                }
                "scope" => {
                    query_pairs_mut
                        .append_pair("scope", v.split(' ').collect::<Vec<_>>().join(",").as_str());
                }
                _ => {
                    query_pairs_mut.append_pair(k.as_str(), v.as_str());
                }
            }
        }
        query_pairs_mut.finish();
    }

    // https://developers.tiktok.com/doc/login-kit-manage-user-access-tokens/
    fn access_token_request_rendering(
        &self,
        body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &TiktokProviderWithWebApplication,
            body: &AccessTokenRequestBody,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let client_key = this.client_id.to_owned();
            let query = TiktokAccessTokenRequestQuery {
                client_key,
                client_secret: this.client_secret.to_owned(),
                code: body.code.to_owned(),
                grant_type: GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT.to_owned(),
            };
            let query_str = serde_qs::to_string(&query)
                .map_err(AccessTokenRequestRenderingError::SerRequestQueryFailed)?;

            let mut url = this.token_endpoint_url().to_owned();
            url.set_query(Some(query_str.as_str()));

            let request = Request::builder()
                .method(Method::POST)
                .uri(url.as_str())
                .body(vec![])
                .map_err(AccessTokenRequestRenderingError::MakeRequestFailed)?;

            Ok(request)
        }

        Some(doing(self, body))
    }

    // https://developers.tiktok.com/doc/login-kit-manage-user-access-tokens/
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
        ) -> Result<TiktokAccessTokenResponseBody, Box<dyn error::Error + Send + Sync + 'static>>
        {
            let body = serde_json::from_slice::<TiktokAccessTokenResponseBody>(response.body())
                .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;

            Ok(body)
        }

        Some(doing(response).map(Into::into))
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct TiktokAccessTokenRequestQuery {
    pub client_key: String,
    pub client_secret: String,
    pub code: String,
    pub grant_type: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenRequestRenderingError {
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
}

//
#[derive(Serialize, Deserialize)]
#[serde(tag = "message", content = "data")]
pub enum TiktokAccessTokenResponseBody {
    #[serde(rename = "success")]
    Success(TiktokAccessTokenResponseBodySuccessfulData),
    #[serde(rename = "error")]
    Error(TiktokAccessTokenResponseBodyErrorData),
}

#[derive(Serialize, Deserialize)]
pub struct TiktokAccessTokenResponseBodySuccessfulData {
    pub open_id: String,
    pub scope: String,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub refresh_expires_in: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TiktokAccessTokenResponseBodyErrorData {
    pub captcha: String,
    pub desc_url: String,
    pub description: String,
    pub error_code: i64,
}

impl From<TiktokAccessTokenResponseBody>
    for Result<AccessTokenResponseSuccessfulBody<TiktokScope>, AccessTokenResponseErrorBody>
{
    fn from(body: TiktokAccessTokenResponseBody) -> Self {
        match body {
            TiktokAccessTokenResponseBody::Success(body) => {
                let scope: Vec<_> = body
                    .scope
                    .split(',')
                    .map(|x| {
                        x.parse::<TiktokScope>()
                            .unwrap_or_else(|_| TiktokScope::Other(x.to_owned()))
                    })
                    .collect();

                let mut map = Map::new();
                map.insert(
                    KEY_OPENID.to_owned(),
                    Value::String(body.open_id.to_owned()),
                );
                map.insert(
                    "refresh_expires_in".to_owned(),
                    Value::Number(body.refresh_expires_in.into()),
                );

                let mut body = AccessTokenResponseSuccessfulBody::<TiktokScope>::new(
                    body.access_token.to_owned(),
                    AccessTokenType::Bearer,
                    Some(body.expires_in as usize),
                    Some(body.refresh_token),
                    if scope.is_empty() {
                        None
                    } else {
                        Some(scope.into())
                    },
                );
                body.set_extra(map);

                Ok(body)
            }
            TiktokAccessTokenResponseBody::Error(body) => {
                let body = AccessTokenResponseErrorBody::new(
                    AccessTokenResponseErrorBodyError::Other(body.error_code.to_string()),
                    Some(body.description),
                    None,
                );

                Err(body)
            }
        }
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
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let provider = TiktokProviderWithWebApplication::new(
            "CLIENT_KEY".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AuthorizationEndpoint::new(
            &provider,
            vec![TiktokScope::UserInfoBasic, TiktokScope::VideoList],
        )
        .configure(|x| x.state = Some("STATE".to_owned()))
        .render_request()?;

        assert_eq!(request.uri(), "https://www.tiktok.com/auth/authorize/?response_type=code&client_key=CLIENT_KEY&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=user.info.basic%2Cvideo.list&state=STATE");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = TiktokProviderWithWebApplication::new(
            "CLIENT_KEY".to_owned(),
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
                    map.get("open_id").unwrap().as_str(),
                    Some("_000fwZ23Mw4RY9cB4lDQyKCgQg4Ft6SyTuE")
                );
            }
            Err(body) => panic!("{body:?}"),
        }

        //
        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_failed_with_authorization_code_grant.json"
        );
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned())
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(body) => {
                panic!("{body:?}")
            }
            Err(body) => assert_eq!(body.error.to_string(), "10007"),
        }

        Ok(())
    }
}
