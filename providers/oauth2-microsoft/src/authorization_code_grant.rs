use oauth2_client::{
    authorization_code_grant::provider_ext::ProviderExtAuthorizationCodeGrantOidcSupportType,
    re_exports::{ClientId, ClientSecret, RedirectUri, Url, UrlParseError},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{authorization_url, token_url, MicrosoftScope};

#[derive(Debug, Clone)]
pub struct MicrosoftProviderForWebApps {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl MicrosoftProviderForWebApps {
    pub fn new(
        tenant: impl AsRef<str>,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            token_endpoint_url: token_url(tenant.as_ref()).parse()?,
            authorization_endpoint_url: authorization_url(tenant.as_ref()).parse()?,
        })
    }
}
impl Provider for MicrosoftProviderForWebApps {
    type Scope = MicrosoftScope;

    fn client_id(&self) -> Option<&ClientId> {
        Some(&self.client_id)
    }

    fn client_secret(&self) -> Option<&ClientSecret> {
        Some(&self.client_secret)
    }

    fn token_endpoint_url(&self) -> &Url {
        &self.token_endpoint_url
    }
}
impl ProviderExtAuthorizationCodeGrant for MicrosoftProviderForWebApps {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn oidc_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantOidcSupportType> {
        Some(ProviderExtAuthorizationCodeGrantOidcSupportType::Yes)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![
            MicrosoftScope::Openid,
            MicrosoftScope::Email,
            MicrosoftScope::Profile,
        ])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }
}
