use std::{error, fmt};

use dyn_clone::{clone_trait_object, DynClone};

use crate::re_exports::Scope;

use super::{AccessTokenResponseSuccessfulBody, GrantInfo, UserInfo, UserInfoEndpointBox};

//
//
//
pub trait Builder<SCOPE>: DynClone
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>>;
}

#[derive(Debug)]
pub enum BuilderObtainUserInfoOutput {
    None,
    Static(UserInfo),
    Respond(UserInfoEndpointBox),
}

//
clone_trait_object!(<SCOPE> Builder<SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> fmt::Debug for dyn Builder<SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Builder").finish()
    }
}

//
//
//
#[derive(Debug, Clone)]
pub struct DefaultBuilder;
impl<SCOPE> Builder<SCOPE> for DefaultBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        _grant_info: GrantInfo<SCOPE>,
        _access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        Ok(BuilderObtainUserInfoOutput::None)
    }
}
