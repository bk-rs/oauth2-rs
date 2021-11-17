use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, Builder, BuilderObtainUserInfoError,
        BuilderObtainUserInfoOutput, GrantInfo,
    },
    re_exports::Scope,
};

use super::MastodonUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct MastodonExtensionsBuilder;

impl<SCOPE> Builder<SCOPE> for MastodonExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, BuilderObtainUserInfoError> {
        let extra = match grant_info {
            GrantInfo::AuthorizationCodeGrant(info) => info.provider.extra(),
            GrantInfo::DeviceAuthorizationGrant(info) => info.provider.extra(),
        };

        let base_url = extra
            .map(|x| x.get("base_url").cloned())
            .ok_or("base_url missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .ok_or("base_url missing")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .as_str()
            .ok_or("base_url mismatch")
            .map_err(BuilderObtainUserInfoError::Unreachable)?
            .to_owned();

        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            MastodonUserInfoEndpoint::new(base_url, &access_token.access_token)
                .map_err(|err| BuilderObtainUserInfoError::Other(Box::new(err)))?,
        )))
    }
}
