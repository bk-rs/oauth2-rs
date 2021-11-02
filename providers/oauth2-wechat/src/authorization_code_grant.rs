use std::{error, str::FromStr};

use oauth2_core::{
    access_token_request::{
        BodyWithAuthorizationCodeGrant, GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
    },
    access_token_response::{ErrorBodyError, GeneralErrorBody, GeneralSuccessfulBody},
    authorization_code_grant::authorization_request::Query,
    provider::{HttpError, Request, Response, Url, UrlParseError},
    types::{AccessTokenType, ClientId, ClientSecret, RedirectUri},
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeJsonError, Map, Value};
use serde_qs::Error as SerdeQsError;

use crate::{WeChatScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct WeChatProviderWithWebApplication {
    appid: ClientId,
    secret: ClientSecret,
    redirect_uri: RedirectUri,
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
            token_endpoint_url: TOKEN_URL.parse()?,
            authorization_endpoint_url: AUTHORIZATION_URL.parse()?,
        })
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
        query: &Query<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn error::Error>>> {
        fn doing(
            query: &Query<<WeChatProviderWithWebApplication as Provider>::Scope>,
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

            let query = AuthorizationRequestQuery {
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

    fn access_token_request_rendering(
        &self,
        body: &BodyWithAuthorizationCodeGrant,
    ) -> Option<Result<Request<Vec<u8>>, Box<dyn error::Error>>> {
        fn doing(
            this: &WeChatProviderWithWebApplication,
            body: &BodyWithAuthorizationCodeGrant,
        ) -> Result<Request<Vec<u8>>, Box<dyn error::Error>> {
            let appid = body
                .client_id
                .to_owned()
                .ok_or_else(|| AccessTokenRequestRenderingError::ClientIdMissing)?;

            let query = AccessTokenRequestQuery {
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
            Result<GeneralSuccessfulBody<<Self as Provider>::Scope>, GeneralErrorBody>,
            Box<dyn error::Error>,
        >,
    > {
        fn doing(
            response: &Response<Vec<u8>>,
        ) -> Result<
            Result<AccessTokenResponseSuccessfulBody, AccessTokenResponseErrorBody>,
            Box<dyn error::Error>,
        > {
            if response.status().is_success() {
                let map = serde_json::from_slice::<Map<String, Value>>(&response.body())
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;
                if !map.contains_key("errcode") {
                    let body = serde_json::from_slice::<AccessTokenResponseSuccessfulBody>(
                        &response.body(),
                    )
                    .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;

                    return Ok(Ok(body));
                }
            }

            let body = serde_json::from_slice::<AccessTokenResponseErrorBody>(&response.body())
                .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;
            Ok(Err(body))
        }

        Some(doing(response).map(|ret| ret.map(|x| x.into()).map_err(|x| x.into())))
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct AuthorizationRequestQuery {
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
pub struct AccessTokenRequestQuery {
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
pub struct AccessTokenResponseSuccessfulBody {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub openid: String,
    pub scope: String,
    pub unionid: Option<String>,
}
impl From<AccessTokenResponseSuccessfulBody> for GeneralSuccessfulBody<WeChatScope> {
    fn from(body: AccessTokenResponseSuccessfulBody) -> Self {
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
        body._extensions = Some(map);

        body
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponseErrorBody {
    pub errcode: usize,
    pub errmsg: String,
}
impl From<AccessTokenResponseErrorBody> for GeneralErrorBody {
    fn from(body: AccessTokenResponseErrorBody) -> Self {
        Self::new(
            ErrorBodyError::Other(body.errcode.to_string()),
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
