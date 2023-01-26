use std::error;

use oauth2_client::{
    authorization_code_grant::provider_ext::{
        AccessTokenRequestBody, AccessTokenResponseErrorBody, AccessTokenResponseSuccessfulBody,
        AuthorizationRequestQuery,
    },
    oauth2_core::{
        access_token_request::GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
        re_exports::AccessTokenResponseErrorBodyError, types::AccessTokenType,
    },
    re_exports::{
        serde_json, serde_qs, thiserror, Body, ClientId, ClientSecret, Deserialize, HttpError, Map,
        RedirectUri, Request, Response, SerdeJsonError, SerdeQsError, Serialize, Url,
        UrlParseError, Value,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{WechatScope, AUTHORIZATION_URL, TOKEN_URL};

pub const KEY_OPENID: &str = "openid";

#[derive(Debug, Clone)]
pub struct WechatProviderWithWebApplication {
    appid: ClientId,
    secret: ClientSecret,
    redirect_uri: RedirectUri,
    pub wechat_redirect: Option<bool>,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl WechatProviderWithWebApplication {
    pub fn new(
        appid: ClientId,
        secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            appid,
            secret,
            redirect_uri,
            wechat_redirect: None,
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
impl Provider for WechatProviderWithWebApplication {
    type Scope = WechatScope;

    fn client_id(&self) -> Option<&ClientId> {
        Some(&self.appid)
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        Some(&self.secret)
    }

    fn token_endpoint_url(&self) -> &Url {
        &self.token_endpoint_url
    }
}
impl ProviderExtAuthorizationCodeGrant for WechatProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![WechatScope::SnsapiLogin])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }

    fn authorization_request_query_serializing(
        &self,
        query: &AuthorizationRequestQuery<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            query: &AuthorizationRequestQuery<
                <WechatProviderWithWebApplication as Provider>::Scope,
            >,
        ) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
            let redirect_uri = query
                .redirect_uri
                .to_owned()
                .ok_or(AuthorizationRequestQuerySerializingError::RedirectUriMissing)?;

            let scope = query
                .scope
                .to_owned()
                .ok_or(AuthorizationRequestQuerySerializingError::ScopeMissing)?;

            let scope = scope
                .0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let query = WechatAuthorizationRequestQuery {
                appid: query.client_id.to_owned(),
                redirect_uri,
                response_type: query.response_type.to_owned(),
                scope,
                state: query.state.to_owned(),
            };

            let query_str = serde_qs::to_string(&query)
                .map_err(AuthorizationRequestQuerySerializingError::SerRequestQueryFailed)?;

            Ok(query_str)
        }

        Some(doing(query))
    }

    fn authorization_request_url_modifying(&self, url: &mut Url) {
        if self.wechat_redirect == Some(true) {
            url.set_fragment(Some("wechat_redirect"));
        }
    }

    fn access_token_request_rendering(
        &self,
        body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &WechatProviderWithWebApplication,
            body: &AccessTokenRequestBody,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let query = WechatAccessTokenRequestQuery {
                appid: this.appid.to_owned(),
                secret: this.secret.to_owned(),
                code: body.code.to_owned(),
                grant_type: GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT.to_owned(),
            };
            let query_str = serde_qs::to_string(&query)
                .map_err(AccessTokenRequestRenderingError::SerRequestQueryFailed)?;

            let mut url = this.token_endpoint_url().to_owned();
            url.set_query(Some(query_str.as_str()));

            let request = Request::builder()
                .uri(url.as_str())
                .body(vec![])
                .map_err(AccessTokenRequestRenderingError::MakeRequestFailed)?;

            Ok(request)
        }

        Some(doing(self, body))
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
            Result<WechatAccessTokenResponseSuccessfulBody, WechatAccessTokenResponseErrorBody>,
            Box<dyn error::Error + Send + Sync + 'static>,
        > {
            if response.status().is_success() {
                let map = serde_json::from_slice::<Map<String, Value>>(response.body())
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;
                if !map.contains_key("errcode") {
                    let body = serde_json::from_slice::<WechatAccessTokenResponseSuccessfulBody>(
                        response.body(),
                    )
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;

                    return Ok(Ok(body));
                }
            }

            let body =
                serde_json::from_slice::<WechatAccessTokenResponseErrorBody>(response.body())
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;
            Ok(Err(body))
        }

        Some(doing(response).map(|ret| ret.map(Into::into).map_err(Into::into)))
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct WechatAuthorizationRequestQuery {
    pub appid: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthorizationRequestQuerySerializingError {
    #[error("RedirectUriMissing")]
    RedirectUriMissing,
    #[error("ScopeMissing")]
    ScopeMissing,
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
}

//
#[derive(Serialize, Deserialize)]
pub struct WechatAccessTokenRequestQuery {
    pub appid: String,
    pub secret: String,
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
pub struct WechatAccessTokenResponseSuccessfulBody {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub openid: String,
    pub scope: String,
}
impl From<WechatAccessTokenResponseSuccessfulBody>
    for AccessTokenResponseSuccessfulBody<WechatScope>
{
    fn from(body: WechatAccessTokenResponseSuccessfulBody) -> Self {
        let scope: Vec<_> = body
            .scope
            .split(',')
            .map(|x| {
                x.parse::<WechatScope>()
                    .unwrap_or_else(|_| WechatScope::Other(x.to_owned()))
            })
            .collect();

        let mut map = Map::new();
        map.insert(KEY_OPENID.to_owned(), Value::String(body.openid.to_owned()));

        let mut body = Self::new(
            body.access_token.to_owned(),
            AccessTokenType::Bearer,
            Some(body.expires_in),
            Some(body.refresh_token),
            if scope.is_empty() {
                None
            } else {
                Some(scope.into())
            },
        );
        body.set_extra(map);

        body
    }
}

#[derive(Serialize, Deserialize)]
pub struct WechatAccessTokenResponseErrorBody {
    pub errcode: usize,
    pub errmsg: String,
}
impl From<WechatAccessTokenResponseErrorBody> for AccessTokenResponseErrorBody {
    fn from(body: WechatAccessTokenResponseErrorBody) -> Self {
        Self::new(
            AccessTokenResponseErrorBodyError::Other(body.errcode.to_string()),
            Some(body.errmsg),
            None,
        )
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
        let provider = WechatProviderWithWebApplication::new(
            "APPID".to_owned(),
            "SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?
        .configure(|x| {
            x.wechat_redirect = Some(true);
        });

        let request = AuthorizationEndpoint::new(&provider, vec![WechatScope::SnsapiLogin])
            .configure(|x| x.state = Some("3d6be0a4035d839573b04816624a415e".to_owned()))
            .render_request()?;

        assert_eq!(request.uri(), "https://open.weixin.qq.com/connect/oauth2/authorize?appid=APPID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&response_type=code&scope=snsapi_login&state=3d6be0a4035d839573b04816624a415e#wechat_redirect");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider = WechatProviderWithWebApplication::new(
            "APPID".to_owned(),
            "SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AccessTokenEndpoint::new(&provider, "CODE".to_owned()).render_request()?;

        assert_eq!(request.method(), "GET");
        assert_eq!(request.uri(), "https://api.weixin.qq.com/sns/oauth2/access_token?appid=APPID&secret=SECRET&code=CODE&grant_type=authorization_code");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = WechatProviderWithWebApplication::new(
            "APPID".to_owned(),
            "SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_with_authorization_code_grant.json"
        );
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned())
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(body) => {
                assert_eq!(body.access_token, "ACCESS_TOKEN");
                assert_eq!(body.token_type, AccessTokenType::Bearer);
                assert_eq!(body.expires_in, Some(7200));
                assert_eq!(body.refresh_token, Some("REFRESH_TOKEN".to_owned()));
                assert_eq!(
                    body.scope,
                    Some(vec![WechatScope::Other("SCOPE".to_owned())].into())
                );
                let map = body.extra().unwrap();
                assert_eq!(map.get("openid").unwrap(), "OPENID");
            }
            Err(body) => panic!("{body:?}"),
        }

        Ok(())
    }
}
