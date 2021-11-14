use std::error;

use oauth2_client::{
    additional_endpoints::{
        AccessTokenResponseSuccessfulBody, BuilderObtainUserInfoOutput, EndpointBuilder, GrantInfo,
    },
    re_exports::Scope,
};

use super::MastodonUserInfoEndpoint;

//
#[derive(Debug, Clone)]
pub struct MastodonEndpointBuilder;

impl<SCOPE> EndpointBuilder<SCOPE> for MastodonEndpointBuilder
where
    SCOPE: Scope,
{
    fn user_info_obtain(
        &self,
        grant_info: GrantInfo<SCOPE>,
        access_token: &AccessTokenResponseSuccessfulBody<SCOPE>,
    ) -> Result<BuilderObtainUserInfoOutput, Box<dyn error::Error + Send + Sync>> {
        let extensions = match grant_info {
            GrantInfo::AuthorizationCodeGrant(info) => info.provider.extensions(),
            GrantInfo::DeviceAuthorizationGrant(info) => info.provider.extensions(),
        };

        let base_url = extensions
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
