use oauth2_client::{
    re_exports::{Body, ClientId, ClientSecret, Request, Url, UrlParseError},
    Provider, ProviderExtClientCredentialsGrant,
};

use crate::{AppleScope, OAUTH2_TOKEN_URL};

//
// https://developer.apple.com/documentation/apple_search_ads/implementing_oauth_for_the_apple_search_ads_api
//
#[derive(Debug, Clone)]
pub struct AppleProviderForSearchAdsApi {
    client_id: ClientId,
    client_secret: ClientSecret,
    //
    token_endpoint_url: Url,
}
impl AppleProviderForSearchAdsApi {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            token_endpoint_url: OAUTH2_TOKEN_URL.parse()?,
        })
    }
}
impl Provider for AppleProviderForSearchAdsApi {
    type Scope = AppleScope;

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
impl ProviderExtClientCredentialsGrant for AppleProviderForSearchAdsApi {
    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![AppleScope::Searchadsorg])
    }

    fn access_token_request_url_modifying(&self, url: &mut Url) {
        let mut query_pairs_mut = url.query_pairs_mut();
        query_pairs_mut.clear();

        query_pairs_mut.append_pair("grant_type", "client_credentials");
        query_pairs_mut.append_pair("scope", AppleScope::Searchadsorg.to_string().as_str());
        query_pairs_mut.append_pair("client_id", &self.client_id);
        query_pairs_mut.append_pair("client_secret", &self.client_secret);

        query_pairs_mut.finish();
    }

    fn access_token_request_modifying(&self, request: &mut Request<Body>) {
        let body = request.body_mut();
        body.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use oauth2_client::{
        client_credentials_grant::AccessTokenEndpoint,
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn access_token_request_for_search_ads_api() -> Result<(), Box<dyn std::error::Error>> {
        let provider = AppleProviderForSearchAdsApi::new(
            "SEARCHADS.27478e71-3bb0-4588-998c-182e2b405577".to_owned(),
            "eyJhbGciOiJFUzI1NiIsImtpZCI6ImJhY2FlYmRhLWUyMTktNDFlZS1hOTA3LWUyYzI1YjI0ZDFiMiJ9.eyJpc3MiOiJTRUFSQ0hBRFMuMjc0NzhlNzEtM2JiMC00NTg4LTk5OGMtMTgyZTJiNDA1NTc3IiwiaWF0IjoxNjU0NDczNjAwLCJleHAiOjE2NzAwMjU2MDAsImF1ZCI6Imh0dHBzOi8vYXBwbGVpZC5hcHBsZS5jb20iLCJzdWIiOiJTRUFSQ0hBRFMuMjc0NzhlNzEtM2JiMC00NTg4LTk5OGMtMTgyZTJiNDA1NTc3In0.bN3KRWDJft-rjqRbOuuzfsImPT4RPEy01ILYJRBe4v_WJtJdi-7xBpi9UCcSN1WRe3Ozobvou5ruxXjVFnB_6Q".to_owned(),
        )?;

        let request = AccessTokenEndpoint::new(&provider, None).render_request()?;

        assert_eq!(request.method(), "POST");
        assert_eq!(
            request.uri(),
            "https://appleid.apple.com/auth/oauth2/token?grant_type=client_credentials&scope=searchadsorg&client_id=SEARCHADS.27478e71-3bb0-4588-998c-182e2b405577&client_secret=eyJhbGciOiJFUzI1NiIsImtpZCI6ImJhY2FlYmRhLWUyMTktNDFlZS1hOTA3LWUyYzI1YjI0ZDFiMiJ9.eyJpc3MiOiJTRUFSQ0hBRFMuMjc0NzhlNzEtM2JiMC00NTg4LTk5OGMtMTgyZTJiNDA1NTc3IiwiaWF0IjoxNjU0NDczNjAwLCJleHAiOjE2NzAwMjU2MDAsImF1ZCI6Imh0dHBzOi8vYXBwbGVpZC5hcHBsZS5jb20iLCJzdWIiOiJTRUFSQ0hBRFMuMjc0NzhlNzEtM2JiMC00NTg4LTk5OGMtMTgyZTJiNDA1NTc3In0.bN3KRWDJft-rjqRbOuuzfsImPT4RPEy01ILYJRBe4v_WJtJdi-7xBpi9UCcSN1WRe3Ozobvou5ruxXjVFnB_6Q"
        );
        assert_eq!(
            request.headers().get("Content-Type").unwrap().as_bytes(),
            b"application/x-www-form-urlencoded"
        );
        assert_eq!(request.body(), b"");

        Ok(())
    }

    #[test]
    fn access_token_response_for_search_ads_api() -> Result<(), Box<dyn std::error::Error>> {
        let provider =
            AppleProviderForSearchAdsApi::new("CLIENT_ID".to_owned(), "CLIENT_SECRET".to_owned())?;

        let response_body =
            include_str!("../tests/response_body_json_files/access_token_for_search_ads_api.json");
        let body_ret = AccessTokenEndpoint::new(&provider, None)
            .parse_response(Response::builder().body(response_body.as_bytes().to_vec())?)?;

        match body_ret {
            Ok(body) => {
                assert_eq!(body.expires_in, Some(3600));
            }
            Err(body) => panic!("{body:?}"),
        }

        Ok(())
    }
}
