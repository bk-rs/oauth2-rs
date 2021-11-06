use std::fmt;

use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::{clone_trait_object, DynClone};

use crate::re_exports::{Body, Request, Response, Scope};

use super::{
    AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointOutputObtainFrom,
    EndpointParseResponseError, EndpointRenderRequestError,
};

//
//
//
pub trait RevokeAccessTokenEndpoint<SCOPE>: DynClone + DowncastSync + Send + Sync
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

clone_trait_object!(<SCOPE> RevokeAccessTokenEndpoint<SCOPE> where SCOPE: self::Scope + Clone);
impl_downcast!(RevokeAccessTokenEndpoint<SCOPE> where SCOPE: self::Scope);

impl<SCOPE> fmt::Debug for dyn RevokeAccessTokenEndpoint<SCOPE>
where
    SCOPE: self::Scope,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RevokeAccessTokenEndpoint").finish()
    }
}
