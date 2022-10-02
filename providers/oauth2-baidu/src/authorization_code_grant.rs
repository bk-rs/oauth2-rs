use std::error;

use oauth2_client::{
    authorization_code_grant::provider_ext::AccessTokenRequestBody,
    oauth2_core::access_token_request::GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT,
    re_exports::{
        serde_qs, thiserror, Body, ClientId, ClientSecret, Deserialize, HttpError, RedirectUri,
        Request, SerdeQsError, Serialize, Url, UrlParseError,
    },
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{BaiduScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct BaiduProviderWithWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl BaiduProviderWithWebApplication {
    pub fn new(
        app_key: ClientId,
        secret_key: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id: app_key,
            client_secret: secret_key,
            redirect_uri,
            token_endpoint_url: TOKEN_URL.parse()?,
            authorization_endpoint_url: AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for BaiduProviderWithWebApplication {
    type Scope = BaiduScope;

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
impl ProviderExtAuthorizationCodeGrant for BaiduProviderWithWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![BaiduScope::Basic])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }

    fn access_token_request_rendering(
        &self,
        body: &AccessTokenRequestBody,
    ) -> Option<Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>>> {
        fn doing(
            this: &BaiduProviderWithWebApplication,
            body: &AccessTokenRequestBody,
        ) -> Result<Request<Body>, Box<dyn error::Error + Send + Sync + 'static>> {
            let query = BaiduAccessTokenRequestBody {
                grant_type: GRANT_TYPE_WITH_AUTHORIZATION_CODE_GRANT.to_owned(),
                code: body.code.to_owned(),
                client_id: this.client_id.to_string(),
                client_secret: this.client_secret.to_string(),
                redirect_uri: this.redirect_uri.to_string(),
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
}

//
#[derive(Serialize, Deserialize)]
pub struct BaiduAccessTokenRequestBody {
    pub grant_type: String,
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenRequestRenderingError {
    #[error("SerRequestQueryFailed {0}")]
    SerRequestQueryFailed(SerdeQsError),
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
        let provider = BaiduProviderWithWebApplication::new(
            "APP_KEY".to_owned(),
            "SECRET_KEY".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request =
            AuthorizationEndpoint::new(&provider, vec![BaiduScope::Basic, BaiduScope::Netdisk])
                .configure(|x| x.state = Some("STATE".to_owned()))
                .render_request()?;

        assert_eq!(request.uri(), "http://openapi.baidu.com/oauth/2.0/authorize?response_type=code&client_id=APP_KEY&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb&scope=basic+netdisk&state=STATE");

        Ok(())
    }

    #[test]
    fn access_token_request() -> Result<(), Box<dyn error::Error>> {
        let provider = BaiduProviderWithWebApplication::new(
            "APP_KEY".to_owned(),
            "SECRET_KEY".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let request = AccessTokenEndpoint::new(&provider, "CODE".to_owned()).render_request()?;

        assert_eq!(request.uri().query().unwrap(), "grant_type=authorization_code&code=CODE&client_id=APP_KEY&client_secret=SECRET_KEY&redirect_uri=https%3A%2F%2Fclient.example.com%2Fcb");

        Ok(())
    }

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = BaiduProviderWithWebApplication::new(
            "APP_KEY".to_owned(),
            "SECRET_KEY".to_owned(),
            RedirectUri::new("https://client.example.com/cb")?,
        )?;

        let response_body = include_str!(
            "../tests/response_body_json_files/access_token_with_authorization_code_grant.json"
        );
        let body_ret = AccessTokenEndpoint::new(&provider, "CODE".to_owned())
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(_body) => {}
            Err(body) => panic!("{:?}", body),
        }

        Ok(())
    }
}
