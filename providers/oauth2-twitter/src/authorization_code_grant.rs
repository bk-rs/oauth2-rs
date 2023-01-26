use std::error;

use oauth2_client::{
    authorization_code_grant::provider_ext::{
        AccessTokenRequestBody, ProviderExtAuthorizationCodeGrantPkceSupportType,
    },
    oauth2_core::access_token_request::GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
    re_exports::{
        http::{header::AUTHORIZATION, Method},
        serde_urlencoded, thiserror, Body, ClientId, ClientSecret, HttpError, RedirectUri, Request,
        SerdeUrlencodedSerError, Url, UrlParseError,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::{Deserialize, Serialize};

use crate::{TwitterScope, AUTHORIZATION_URL, TOKEN_URL};

//
#[derive(Debug, Clone)]
pub struct TwitterProviderWithWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl TwitterProviderWithWebApplication {
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
impl Provider for TwitterProviderWithWebApplication {
    type Scope = TwitterScope;

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
impl ProviderExtAuthorizationCodeGrant for TwitterProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn pkce_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantPkceSupportType> {
        Some(ProviderExtAuthorizationCodeGrantPkceSupportType::Yes)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![
            TwitterScope::TweetRead,
            TwitterScope::TweetWrite,
            TwitterScope::OfflineAccess,
        ])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }

    // https://developer.twitter.com/en/docs/authentication/oauth-2-0/user-access-token
    fn access_token_request_rendering(
        &self,
        body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &TwitterProviderWithWebApplication,
            body: &AccessTokenRequestBody,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let body = TwitterAccessTokenRequestBody {
                grant_type: GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT.to_owned(),
                code: body.code.to_owned(),
                redirect_uri: this.redirect_uri.to_string(),
                code_verifier: body
                    .code_verifier
                    .to_owned()
                    .ok_or(AccessTokenRequestRenderingError::CodeVerifierMissing)?,
            };
            let body_str = serde_urlencoded::to_string(&body)
                .map_err(AccessTokenRequestRenderingError::SerRequestBodyFailed)?;

            let url = this.token_endpoint_url().to_owned();

            let request = Request::builder()
                .method(Method::POST)
                .uri(url.as_str())
                .header(
                    AUTHORIZATION,
                    http_authentication::Credentials::basic(&this.client_id, &this.client_secret)
                        .to_string(),
                )
                .body(body_str.as_bytes().to_vec())
                .map_err(AccessTokenRequestRenderingError::MakeRequestFailed)?;

            Ok(request)
        }

        Some(doing(self, body))
    }
}

//
#[derive(Serialize, Deserialize)]
pub struct TwitterAccessTokenRequestBody {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub code_verifier: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenRequestRenderingError {
    #[error("CodeVerifierMissing")]
    CodeVerifierMissing,
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeUrlencodedSerError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
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
        let provider = TwitterProviderWithWebApplication::new(
            "CLIENT_ID".to_owned(),
            "CLIENT_SECRET".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AuthorizationEndpoint::new(
            &provider,
            vec![TwitterScope::TweetRead, TwitterScope::UsersRead],
        )
        .configure(|x| x.state = Some("STATE".to_owned()))
        .render_request()?;

        assert_eq!(request.uri(), "https://twitter.com/i/oauth2/authorize?response_type=code&client_id=CLIENT_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=tweet.read+users.read&state=STATE");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = TwitterProviderWithWebApplication::new(
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
                assert_eq!(body.expires_in, Some(7200));
            }
            Err(body) => panic!("{body:?}"),
        }

        Ok(())
    }
}
