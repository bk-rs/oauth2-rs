use std::fmt;

use dyn_clone::{clone_trait_object, DynClone};

use crate::re_exports::{Endpoint, Scope};

use super::{
    AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, EndpointParseResponseError,
    EndpointRenderRequestError, UserInfo,
};

//
//
//
pub trait EndpointBuilder<SCOPE>: DynClone
where
    SCOPE: Scope,
{
    fn user_info_endpoint_build(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> UserInfoEndpointBuildOutput;
}

clone_trait_object!(<SCOPE> EndpointBuilder<SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> fmt::Debug for dyn EndpointBuilder<SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EndpointBuilder").finish()
    }
}

//
//
//
#[derive(Debug)]
pub enum UserInfoEndpointBuildOutput {
    None,
    Static(UserInfo),
    Respond(
        Box<
            dyn Endpoint<
                    RenderRequestError = EndpointRenderRequestError,
                    ParseResponseOutput = UserInfo,
                    ParseResponseError = EndpointParseResponseError,
                > + Send
                + Sync,
        >,
    ),
}
