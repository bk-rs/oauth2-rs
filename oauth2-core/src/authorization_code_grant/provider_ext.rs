use crate::{
    provider::{Map, Url, Value},
    types::RedirectUri,
    Provider,
};

pub trait ProviderExtAuthorizationCodeGrant: Provider {
    fn redirect_uri(&self) -> Option<RedirectUri>;

    fn authorization_endpoint_url(&self) -> Url;

    fn authorization_request_query_extensions(&self) -> Option<Map<String, Value>> {
        None
    }

    fn access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        None
    }
}
