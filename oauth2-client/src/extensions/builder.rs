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
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError>;
}

#[derive(Debug)]
pub enum BuilderObtainUserInfoOutput {
    None,
    Static(UserInfo),
    Respond(UserInfoEndpointBox),
}

#[derive(thiserror::Error, Debug)]
pub enum BuilderObtainUserInfoError {
    //
    #[error("Unreachable {0}")]
    Unreachable(&'static str),
    //
    #[error("Other {0}")]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

//
clone_trait_object!(<SCOPE> Builder<SCOPE> where SCOPE: Scope + Clone);

impl<SCOPE> core::fmt::Debug for dyn Builder<SCOPE> + Send + Sync
where
    SCOPE: Scope,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        Ok(BuilderObtainUserInfoOutput::None)
    }
}
