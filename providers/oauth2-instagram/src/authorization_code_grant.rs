use oauth2_client::{
    re_exports::{ClientId, ClientSecret, RedirectUri, Url, UrlParseError},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{InstagramScope, AUTHORIZATION_URL, TOKEN_URL};

#[derive(Debug, Clone)]
pub struct InstagramProviderForBasicDisplayApi {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl InstagramProviderForBasicDisplayApi {
    pub fn new(
        instagram_app_id: ClientId,
        instagram_app_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id: instagram_app_id,
            client_secret: instagram_app_secret,
            redirect_uri,
            token_endpoint_url: TOKEN_URL.parse()?,
            authorization_endpoint_url: AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for InstagramProviderForBasicDisplayApi {
    type Scope = InstagramScope;

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
impl ProviderExtAuthorizationCodeGrant for InstagramProviderForBasicDisplayApi {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![InstagramScope::UserProfile, InstagramScope::UserMedia])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use oauth2_client::{
        authorization_code_grant::AccessTokenEndpoint,
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn access_token_response() -> Result<(), Box<dyn error::Error>> {
        let provider = InstagramProviderForBasicDisplayApi::new(
            "APP_ID".to_owned(),
            "APP_SECRET".to_owned(),
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
                    map.get("user_id").unwrap().as_u64(),
                    Some(17841403401953170)
                );
            }
            Err(body) => panic!("{body:?}"),
        }

        Ok(())
    }
}
