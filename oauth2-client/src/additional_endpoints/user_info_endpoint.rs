use dyn_clone::{clone_trait_object, DynClone};

use crate::re_exports::{AccessTokenResponseSuccessfulBody, Body, Request, Response, Scope};

use super::{
    AccessTokenObtainFrom, EndpointParseResponseError, EndpointRenderRequestError, UserInfo,
};

pub trait UserInfoEndpoint<SCOPE>: DynClone
where
    SCOPE: Scope,
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
    ) -> Result<UserInfo, EndpointParseResponseError>;
}

clone_trait_object!(<SCOPE> UserInfoEndpoint<SCOPE> where SCOPE: Clone);
