use std::error;

use oauth2_client::{
    extensions::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, ExtensionsBuilder,
        GrantInfo,
    },
    re_exports::Scope,
};

use super::MastodonUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct MastodonExtensionsBuilder;

impl<SCOPE> ExtensionsBuilder<SCOPE> for MastodonExtensionsBuilder
where
    SCOPE: Scope,
{
    fn obtain_user_info(
        &self,
        grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        let extra = match grant_info {
            GrantInfo::AuthorizationCodeGrant(info) => info.provider.extra(),
            GrantInfo::DeviceAuthorizationGrant(info) => info.provider.extra(),
        };

        let base_url = extra
            .map(|x| x.get("base_url").cloned())
            .ok_or("Missing base_url")?
            .ok_or("Missing base_url")?
            .as_str()
            .ok_or("Mismatch base_url")?
            .to_owned();

        Ok(BuilderObtainUserInfoOutput::Respond(Box::new(
            MastodonUserInfoEndpoint::new(base_url, &access_token.access_token)?,
        )))
    }
}
