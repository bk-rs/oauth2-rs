use std::error;

use oauth2_client::{
    authorization_code_grant::provider_ext::AccessTokenRequestBody,
    oauth2_core::access_token_request::GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
    re_exports::{
        http::{header::AUTHORIZATION, Method},
        serde_urlencoded, thiserror, Body, ClientId, ClientSecret, HttpError, RedirectUri, Request,
        SerdeUrlencodedSerError, Url, UrlParseError,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};
use serde::{Deserialize, Serialize};

use crate::{PinterestScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct PinterestProviderWithWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl PinterestProviderWithWebApplication {
    pub fn new(
        app_id: ClientId,
        app_secret_key: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id: app_id,
            client_secret: app_secret_key,
            redirect_uri,
            token_endpoint_url: TOKEN_URL.parse()?,
            authorization_endpoint_url: AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for PinterestProviderWithWebApplication {
    type Scope = PinterestScope;

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
impl ProviderExtAuthorizationCodeGrant for PinterestProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![
            PinterestScope::BoardsRead,
            PinterestScope::BoardsWrite,
            PinterestScope::PinsRead,
            PinterestScope::PinsWrite,
            PinterestScope::UserAccountsRead,
        ])
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

    // https://developers.pinterest.com/docs/getting-started/authentication/#Exchange%20the%20code%20for%20an%20access%20token
    // https://developers.pinterest.com/docs/api/v5/#operation/oauth/token
    fn access_token_request_rendering(
        &self,
        body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &PinterestProviderWithWebApplication,
            body: &AccessTokenRequestBody,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let body = PinterestAccessTokenRequestBody {
                grant_type: GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT.to_owned(),
                code: body.code.to_owned(),
                redirect_uri: this.redirect_uri.to_string(),
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
pub struct PinterestAccessTokenRequestBody {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenRequestRenderingError {
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
        let provider = PinterestProviderWithWebApplication::new(
            "APP_ID".to_owned(),
            "APP_SECRET_KEY".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AuthorizationEndpoint::new(
            &provider,
            vec![PinterestScope::BoardsRead, PinterestScope::PinsRead],
        )
        .configure(|x| x.state = Some("STATE".to_owned()))
        .render_request()?;

        assert_eq!(request.uri(), "https://www.pinterest.com/oauth/?response_type=code&client_id=APP_ID&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=boards%3Aread%2Cpins%3Aread&state=STATE");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = PinterestProviderWithWebApplication::new(
            "APP_ID".to_owned(),
            "APP_SECRET_KEY".to_owned(),
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
                    map.get("refresh_token_expires_in").unwrap().as_u64(),
                    Some(31536000)
                );
            }
            Err(body) => panic!("{:?}", body),
        }

        Ok(())
    }
}
