use oauth2_client::{
    oauth2_core::{
        re_exports::{AccessTokenResponseErrorBody, AccessTokenResponseSuccessfulBody},
        types::{AccessTokenType, ScopeParameter},
    },
    re_exports::{
        serde_json, thiserror, Body, ClientId, ClientSecret, RedirectUri, Response, SerdeJsonError,
        Url, UrlParseError,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::{Deserialize, Serialize};

use crate::{LinodeScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct LinodeProviderWithWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl LinodeProviderWithWebApplication {
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
impl Provider for LinodeProviderWithWebApplication {
    type Scope = LinodeScope;

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
impl ProviderExtAuthorizationCodeGrant for LinodeProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![LinodeScope::AccountReadOnly])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
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
            Box<dyn std::error::Error + Send + Sync + 'static>,
        >,
    > {
        fn doing(
            response: &Response<Body>,
        ) -> Result<LinodeAccessTokenResponseBody, Box<dyn std::error::Error + Send + Sync + 'static>>
        {
            let body = serde_json::from_slice::<LinodeAccessTokenResponseBody>(response.body())
                .map_err(AccessTokenResponseParsingError::DeResponseBodyFailed)?;

            Ok(body)
        }

        Some(doing(response).map(Into::into))
    }
}

#[derive(Serialize, Deserialize)]
pub struct LinodeAccessTokenResponseBody {
    pub access_token: String,
    pub token_type: AccessTokenType,
    pub expires_in: Option<usize>,
    pub refresh_token: Option<String>,
    pub scope: Option<ScopeParameter<LinodeScope>>,
}

impl From<LinodeAccessTokenResponseBody>
    for Result<AccessTokenResponseSuccessfulBody<LinodeScope>, AccessTokenResponseErrorBody>
{
    fn from(body: LinodeAccessTokenResponseBody) -> Self {
        Ok(AccessTokenResponseSuccessfulBody {
            access_token: body.access_token.to_owned(),
            token_type: body.token_type,
            expires_in: body.expires_in,
            refresh_token: body.refresh_token.to_owned(),
            scope: body.scope,
            id_token: None,
            _extra: None,
        })
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

    use oauth2_client::{
        authorization_code_grant::AccessTokenEndpoint,
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn access_token_response() -> Result<(), Box<dyn std::error::Error>> {
        let provider = LinodeProviderWithWebApplication::new(
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
            Ok(_body) => {}
            Err(body) => panic!("{body:?}"),
        }

        Ok(())
    }
}
