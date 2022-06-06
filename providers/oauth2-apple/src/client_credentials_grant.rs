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

    use std::error;

    use oauth2_client::{
        client_credentials_grant::AccessTokenEndpoint,
        re_exports::{Endpoint as _, Response},
    };

    #[test]
    fn access_token_request_for_search_ads_api() -> Result<(), Box<dyn error::Error>> {
        let provider = AppleProviderForSearchAdsApi::new(
            "SEARCHADS.27478e71-3bb0-4588-998c-182e2b405577".to_owned(),
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.zI1NiIsImprdSI6Imh0dHBzOi8vYXV0aC5kZXYuYXBpLnJpY29oL3YxL2Rpc2NvdmVyeS9rZXlzIiwia2lkIjoiMmIyZTgyMTA2NzkxZGM4ZmFkNzgxNWQ3ZmI1NDRhNjJmNzJjMTZmYSJ9.eyJpc3MiOiJodHRwczovL2F1dGguZGV2LmFwaS5yaWNvaC8iLCJhdWQiOiJodHRwczovL2lwcy5kZXYuYXBpLnJpY29oLyIsImlhdCI6MTQ5MDg1Mjc0MSwiZXhwIjoxNDkwODU2MzQxLCJjbGllbnRfaWQiOiI4ODQwMWU1MS05MzliLTQ3NzktYjdmNy03YzlmNGIzZjkyYzAiLCJzY29wZSI6Imh0dHBzOi8vaXBzLmRldi5hcGkucmljb2gvdjEiLCJyaWNvaF9tc3MiOnsibWVkaWEiOnsicXVvdGEiOjEwLCJ0aHJvdHRsZSI6eyJ2YWx1ZSI6MCwid2luZG93IjowfX19fQ.jVq_c_cTzgsLipkJKBjAHzm8KDehW4rFA1Yg0EQRmqWmBDlEKtpRpDHZeF6ZSQfNH2OlrBWFBiVDV9Th091QFEYrZETZ1IE1koAO14oj4kf8TCmhiG_CtJagvctvloW1wAdgMB1_Eubz9a8oimcODqL7_uTmA5jKFx3ez9uoqQrEKZ51g665jSI6NlyeLtj4LrxpI9jZ4zTx1yqqjQx0doYQjBPhOB06Z5bdiVyhJDRpE8ksRCC3kDPS2nsvDAal28sMgyeP8sPvfKvp5sa2UsH78WJmTzeZWcJfX2C2ba3xwRMB5LaaVrQZlhj9xjum0MfDpIS1hJI6p5CHZ8w".to_owned(),
        )?;

        let request = AccessTokenEndpoint::new(&provider, None).render_request()?;

        assert_eq!(request.method(), "POST");
        assert_eq!(
            request.uri(),
            "https://appleid.apple.com/auth/oauth2/token?grant_type=client_credentials&scope=searchadsorg&client_id=SEARCHADS.27478e71-3bb0-4588-998c-182e2b405577&client_secret=eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.zI1NiIsImprdSI6Imh0dHBzOi8vYXV0aC5kZXYuYXBpLnJpY29oL3YxL2Rpc2NvdmVyeS9rZXlzIiwia2lkIjoiMmIyZTgyMTA2NzkxZGM4ZmFkNzgxNWQ3ZmI1NDRhNjJmNzJjMTZmYSJ9.eyJpc3MiOiJodHRwczovL2F1dGguZGV2LmFwaS5yaWNvaC8iLCJhdWQiOiJodHRwczovL2lwcy5kZXYuYXBpLnJpY29oLyIsImlhdCI6MTQ5MDg1Mjc0MSwiZXhwIjoxNDkwODU2MzQxLCJjbGllbnRfaWQiOiI4ODQwMWU1MS05MzliLTQ3NzktYjdmNy03YzlmNGIzZjkyYzAiLCJzY29wZSI6Imh0dHBzOi8vaXBzLmRldi5hcGkucmljb2gvdjEiLCJyaWNvaF9tc3MiOnsibWVkaWEiOnsicXVvdGEiOjEwLCJ0aHJvdHRsZSI6eyJ2YWx1ZSI6MCwid2luZG93IjowfX19fQ.jVq_c_cTzgsLipkJKBjAHzm8KDehW4rFA1Yg0EQRmqWmBDlEKtpRpDHZeF6ZSQfNH2OlrBWFBiVDV9Th091QFEYrZETZ1IE1koAO14oj4kf8TCmhiG_CtJagvctvloW1wAdgMB1_Eubz9a8oimcODqL7_uTmA5jKFx3ez9uoqQrEKZ51g665jSI6NlyeLtj4LrxpI9jZ4zTx1yqqjQx0doYQjBPhOB06Z5bdiVyhJDRpE8ksRCC3kDPS2nsvDAal28sMgyeP8sPvfKvp5sa2UsH78WJmTzeZWcJfX2C2ba3xwRMB5LaaVrQZlhj9xjum0MfDpIS1hJI6p5CHZ8w"
        );
        assert_eq!(
            request.headers().get("Content-Type").unwrap().as_bytes(),
            b"application/x-www-form-urlencoded"
        );
        assert_eq!(request.body(), b"");

        Ok(())
    }

    #[test]
    fn access_token_response_for_search_ads_api() -> Result<(), Box<dyn error::Error>> {
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
            Err(body) => panic!("{:?}", body),
        }

        Ok(())
    }
}
