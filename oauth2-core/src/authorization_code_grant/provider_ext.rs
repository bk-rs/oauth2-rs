use std::{error, fmt, str};

use crate::{
    provider::{Map, Url, Value},
    types::RedirectUri,
    Provider,
};

use super::authorization_request::Query;

pub trait ProviderExtAuthorizationCodeGrant: Provider
where
    <<Self as Provider>::Scope as str::FromStr>::Err: fmt::Display,
{
    fn redirect_uri(&self) -> Option<&RedirectUri>;

    fn authorization_endpoint_url(&self) -> &Url;

    fn authorization_request_query_extensions(&self) -> Option<Map<String, Value>> {
        None
    }
    fn authorization_request_query_serializer(
        &self,
        _query: &Query<<Self as Provider>::Scope>,
    ) -> Option<Result<String, Box<dyn error::Error>>> {
        None
    }

    fn access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        None
    }
}
