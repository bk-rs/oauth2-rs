use std::error;

use crate::re_exports::{Body, Request, Response, Scope};

use super::{
    AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointOutputObtainFrom,
    EndpointParseResponseError, EndpointRenderRequestError,
};

//
//
//
pub trait RefreshAccessTokenEndpoint<SCOPE>
where
    SCOPE: Scope,
{
    fn obtain_from(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> EndpointOutputObtainFrom {
        EndpointOutputObtainFrom::Respond
    }

    fn build(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<AccessTokenResponseSuccessfulBody<SCOPE>, Box<dyn error::Error + 'static>> {
        unimplemented!()
    }

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
    ) -> Result<AccessTokenResponseSuccessfulBody<SCOPE>, EndpointParseResponseError>;
}
