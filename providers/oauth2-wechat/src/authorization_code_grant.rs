use std::error;

use oauth2_core::{
    access_token_request::{
        BodyWithAuthorizationCodeGrant, GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
    },
    authorization_code_grant::authorization_request::Query,
    provider::{Url, UrlParseError},
    types::{ClientId, ClientSecret, RedirectUri},
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::{Deserialize, Serialize};
use serde_qs::Error as SerdeQsError;
use serde_urlencoded::ser::Error as SerdeUrlencodedSerError;

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

    fn authorization_request_query_serializer(
        &self,
        query: &Query<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn error::Error>>> {
        let redirect_uri = if let Some(redirect_uri) = &query.redirect_uri {
            redirect_uri.to_string()
        } else {
            return Some(Err(Box::new(
                AuthorizationRequestQuerySerializerError::RedirectUriMissing,
            )));
        };

        let scope = if let Some(scope) = &query.scope {
            scope
                .0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        } else {
            return Some(Err(Box::new(
                AuthorizationRequestQuerySerializerError::ScopeMissing,
            )));
        };

        let query = AuthorizationRequestQuery {
            appid: query.client_id.to_owned(),
            redirect_uri,
            response_type: query.response_type.to_owned(),
            scope,
            state: query.state.to_owned(),
        };

        let query_str = match serde_qs::to_string(&query) {
            Ok(x) => x,
            Err(err) => {
                return Some(Err(Box::new(
                    AuthorizationRequestQuerySerializerError::SerRequestQueryFailed(err),
                )))
            }
        };

        Some(Ok(query_str))
    }

    fn access_token_request_body_serializer(
        &self,
        body: &BodyWithAuthorizationCodeGrant,
    ) -> Option<Result<String, Box<dyn error::Error>>> {
        let appid = if let Some(client_id) = &body.client_id {
            client_id.to_owned()
        } else {
            return Some(Err(Box::new(
                AccessTokenRequestBodySerializerError::ClientIdMissing,
            )));
        };

        let body = AccessTokenRequestBody {
            appid,
            secret: self.secret.to_owned(),
            code: body.code.to_owned(),
            grant_type: GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT.to_owned(),
        };

        let query_str = match serde_urlencoded::to_string(&body) {
            Ok(x) => x,
            Err(err) => {
                return Some(Err(Box::new(
                    AccessTokenRequestBodySerializerError::SerRequestBodyFailed(err),
                )))
            }
        };

        Some(Ok(query_str))
    }
}

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
pub enum AuthorizationRequestQuerySerializerError {
    #[error("RedirectUriMissing")]
    RedirectUriMissing,
    #[error("ScopeMissing")]
    ScopeMissing,
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenRequestBody {
    pub appid: String,
    pub secret: String,
    pub code: String,
    pub grant_type: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenRequestBodySerializerError {
    #[error("ClientIdMissing")]
    ClientIdMissing,
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
}
