use std::{error, fmt};

use dyn_clone::{clone_trait_object, DynClone};

use crate::re_exports::Scope;

use super::{AccessTokenObtainFrom, AccessTokenResponseSuccessfulBody, UserInfoObtainOutput};

//
//
//
pub trait EndpointBuilder<SCOPE>: DynClone
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        _access_token_obtain_from: AccessTokenObtainFrom,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<UserInfoObtainOutput, Box<dyn error::Error + Send + Sync>>;
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
