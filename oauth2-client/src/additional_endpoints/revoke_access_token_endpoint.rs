use std::{fmt, str};

use crate::re_exports::{AccessTokenResponseSuccessfulBody, Body, Request, Response, Scope};

use super::{AccessTokenObtainFrom, EndpointParseResponseError, EndpointRenderRequestError};

pub trait RevokeAccessTokenEndpoint<SCOPE>
where
    SCOPE: Scope,
    <SCOPE as str::FromStr>::Err: fmt::Display,
{
    fn can_execute(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> bool;

    fn render_request(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<Request<Body>, EndpointRenderRequestError>;

    fn parse_response(
        &self,
        access_token_obtain_from: AccessTokenObtainFrom,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
        response: Response<Body>,
    ) -> Result<(), EndpointParseResponseError>;
}
