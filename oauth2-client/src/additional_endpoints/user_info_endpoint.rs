use std::{error, fmt};

use dyn_clone::{clone_trait_object, DynClone};

use crate::re_exports::{Body, Request, Response, Scope};

use super::{
    AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointOutputObtainFrom,
    EndpointParseResponseError, EndpointRenderRequestError, UserInfo,
};

//
//
//
pub trait UserInfoEndpoint<SCOPE>: DynClone
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
    ) -> Result<UserInfo, Box<dyn error::Error + 'static>> {
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
    ) -> Result<UserInfo, EndpointParseResponseError>;
}

clone_trait_object!(<SCOPE> UserInfoEndpoint<SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> fmt::Debug for dyn UserInfoEndpoint<SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserInfoEndpoint").finish()
    }
}

//
//
//
#[derive(Debug, Clone)]
pub struct DefaultUserInfoEndpoint;

impl<SCOPE> UserInfoEndpoint<SCOPE> for DefaultUserInfoEndpoint
where
    SCOPE: Scope,
{
    fn obtain_from(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> EndpointOutputObtainFrom {
        EndpointOutputObtainFrom::None
    }

    fn render_request(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<Request<Body>, EndpointRenderRequestError> {
        unreachable!()
    }

    fn parse_response(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
        _response: Response<Body>,
    ) -> Result<UserInfo, EndpointParseResponseError> {
        unreachable!()
    }
}
