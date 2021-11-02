use std::{error, str::FromStr};

use oauth2_client::{
    authorization_code_grant::provider_ext::{
        serde_qs, AccessTokenRequestBody, AccessTokenResponseErrorBody,
        AccessTokenResponseErrorBodyError, AccessTokenResponseSuccessfulBody,
        AuthorizationRequestQuery, SerdeQsError, GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
    },
    provider::{
        serde::{Deserialize, Serialize},
        serde_json, thiserror, AccessTokenType, ClientId, ClientSecret, HttpError, Map,
        RedirectUri, Request, Response, SerdeJsonError, Url, UrlParseError, Value,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{WeChatScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct WeChatProviderWithWebApplication {
    appid: ClientId,
    secret: ClientSecret,
    redirect_uri: RedirectUri,
    wechat_redirect: Option<bool>,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl WeChatProviderWithWebApplication {
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

    pub fn enable_wechat_redirect(&mut self) {
        self.wechat_redirect = Some(true);
    }
}
impl Provider for WeChatProviderWithWebApplication {
    type Scope = WeChatScope;

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
impl ProviderExtAuthorizationCodeGrant for WeChatProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }

    fn authorization_request_query_serializing(
        &self,
        query: &AuthorizationRequestQuery<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn error::Error>>> {
        fn doing(
            query: &AuthorizationRequestQuery<
                <WeChatProviderWithWebApplication as Provider>::Scope,
            >,
        ) -> Result<String, Box<dyn error::Error>> {
            let redirect_uri = query
                .redirect_uri
                .to_owned()
                .ok_or_else(|| AuthorizationRequestQuerySerializingError::RedirectUriMissing)?;

            let redirect_uri = redirect_uri.to_string();

            let scope = query
                .scope
                .to_owned()
                .ok_or_else(|| AuthorizationRequestQuerySerializingError::ScopeMissing)?;

            let scope = scope
                .0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let query = MyAuthorizationRequestQuery {
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
    ) -> Option<Result<Request<Vec<u8>>, Box<dyn error::Error>>> {
        fn doing(
            this: &WeChatProviderWithWebApplication,
            body: &AccessTokenRequestBody,
        ) -> Result<Request<Vec<u8>>, Box<dyn error::Error>> {
            let appid = body
                .client_id
                .to_owned()
                .ok_or_else(|| AccessTokenRequestRenderingError::ClientIdMissing)?;

            let query = MyAccessTokenRequestQuery {
                appid,
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

    fn access_token_response_parsing(
        &self,
        response: &Response<Vec<u8>>,
    ) -> Option<
        Result<
            Result<
                AccessTokenResponseSuccessfulBody<<Self as Provider>::Scope>,
                AccessTokenResponseErrorBody,
            >,
            Box<dyn error::Error>,
        >,
    > {
        fn doing(
            response: &Response<Vec<u8>>,
        ) -> Result<
            Result<MyAccessTokenResponseSuccessfulBody, MyAccessTokenResponseErrorBody>,
            Box<dyn error::Error>,
        > {
            if response.status().is_success() {
                let map = serde_json::from_slice::<Map<String, Value>>(&response.body())
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;
                if !map.contains_key("errcode") {
                    let body = serde_json::from_slice::<MyAccessTokenResponseSuccessfulBody>(
                        &response.body(),
                    )
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;

                    return Ok(Ok(body));
                }
            }

            let body = serde_json::from_slice::<MyAccessTokenResponseErrorBody>(&response.body())
                .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;
            Ok(Err(body))
        }

        Some(doing(response).map(|ret| ret.map(|x| x.into()).map_err(|x| x.into())))
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct MyAuthorizationRequestQuery {
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
pub struct MyAccessTokenRequestQuery {
    pub appid: String,
    pub secret: String,
    pub code: String,
    pub grant_type: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenRequestRenderingError {
    #[error("ClientIdMissing")]
    ClientIdMissing,
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
}

//
#[derive(Serialize, Deserialize)]
pub struct MyAccessTokenResponseSuccessfulBody {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub openid: String,
    pub scope: String,
    pub unionid: Option<String>,
}
impl From<MyAccessTokenResponseSuccessfulBody> for AccessTokenResponseSuccessfulBody<WeChatScope> {
    fn from(body: MyAccessTokenResponseSuccessfulBody) -> Self {
        let scope: Vec<_> = body
            .scope
            .split(',')
            .map(|x| WeChatScope::from_str(x).unwrap_or(WeChatScope::Other(x.to_owned())))
            .collect();

        let mut map = Map::new();
        map.insert("openid".to_owned(), Value::String(body.openid.to_owned()));
        if let Some(unionid) = &body.unionid {
            map.insert("unionid".to_owned(), Value::String(unionid.to_owned()));
        }

        let mut body = Self::new(
            body.access_token.to_owned(),
            AccessTokenType::Bearer,
            Some(body.expires_in),
            Some(body.refresh_token.to_owned()),
            if scope.is_empty() {
                None
            } else {
                Some(scope.into())
            },
        );
        body.set_extensions(map);

        body
    }
}

#[derive(Serialize, Deserialize)]
pub struct MyAccessTokenResponseErrorBody {
    pub errcode: usize,
    pub errmsg: String,
}
impl From<MyAccessTokenResponseErrorBody> for AccessTokenResponseErrorBody {
    fn from(body: MyAccessTokenResponseErrorBody) -> Self {
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
        http_api_endpoint::Endpoint as _,
        provider::Response,
    };

    #[test]
    fn authorization_request() -> Result<(), Box<dyn error::Error>> {
        let mut provider = WeChatProviderWithWebApplication::new(
            "APPID".to_owned(),
            "SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;
        provider.enable_wechat_redirect();

        let endpoint = AuthorizationEndpoint::new(
            &provider,
            vec![WeChatScope::SnsapiLogin],
            "3d6be0a4035d839573b04816624a415e".to_owned(),
        );

        let request = endpoint.render_request()?;

        assert_eq!(request.uri(), "https://open.weixin.qq.com/connect/qrconnect?appid=APPID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&response_type=code&scope=snsapi_login&state=3d6be0a4035d839573b04816624a415e#wechat_redirect");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider = WeChatProviderWithWebApplication::new(
            "APPID".to_owned(),
            "SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let endpoint = AccessTokenEndpoint::new(&provider, "CODE".to_owned());

        let request = endpoint.render_request()?;

        assert_eq!(request.uri(), "https://api.weixin.qq.com/sns/oauth2/access_token?appid=APPID&secret=SECRET&code=CODE&grant_type=authorization_code");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = WeChatProviderWithWebApplication::new(
            "APPID".to_owned(),
            "SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let endpoint = AccessTokenEndpoint::new(&provider, "CODE".to_owned());

        let response_body = r#"
        { 
            "access_token":"ACCESS_TOKEN", 
            "expires_in":7200, 
            "refresh_token":"REFRESH_TOKEN",
            "openid":"OPENID", 
            "scope":"SCOPE",
            "unionid": "o6_bmasdasdsad6_2sgVt7hMZOPfL"
        }"#;
        let body_ret = endpoint
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(body) => {
                assert_eq!(body.access_token, "ACCESS_TOKEN");
                assert_eq!(
                    body.scope,
                    Some(vec![WeChatScope::Other("SCOPE".to_owned())].into())
                );
                let map = body.extensions().unwrap();
                assert_eq!(map.get("openid").unwrap(), "OPENID");
                assert_eq!(map.get("unionid").unwrap(), "o6_bmasdasdsad6_2sgVt7hMZOPfL");
            }
            Err(body) => panic!("{:?}", body),
        }

        Ok(())
    }
}
