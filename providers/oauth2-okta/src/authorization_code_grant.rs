use oauth2_client::{
    authorization_code_grant::provider_ext::{
        ProviderExtAuthorizationCodeGrantOidcSupportType,
        ProviderExtAuthorizationCodeGrantPkceSupportType,
    },
    re_exports::{ClientId, ClientSecret, RedirectUri, Url, UrlParseError},
    Provider, ProviderExtAuthorizationCodeGrant,
};

use crate::{authorization_url, token_url, OktaScope};

#[derive(Debug, Clone)]
pub struct OktaProviderForWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl OktaProviderForWebApplication {
    pub fn new(
        domain: impl AsRef<str>,
        authorization_server_id: impl Into<Option<String>>,
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
    ) -> Result<Self, UrlParseError> {
        let authorization_server_id = authorization_server_id.into();

        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            token_endpoint_url: token_url(domain.as_ref(), authorization_server_id.to_owned())
                .parse()?,
            authorization_endpoint_url: authorization_url(domain.as_ref(), authorization_server_id)
                .parse()?,
        })
    }
}
impl Provider for OktaProviderForWebApplication {
    type Scope = OktaScope;

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
impl ProviderExtAuthorizationCodeGrant for OktaProviderForWebApplication {
    fn redirect_uri(&self) -> Option<&RedirectUri> {
        Some(&self.redirect_uri)
    }

    fn oidc_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantOidcSupportType> {
        Some(ProviderExtAuthorizationCodeGrantOidcSupportType::Yes)
    }

    fn pkce_support_type(&self) -> Option<ProviderExtAuthorizationCodeGrantPkceSupportType> {
        Some(ProviderExtAuthorizationCodeGrantPkceSupportType::Yes)
    }

    fn scopes_default(&self) -> Option<Vec<<Self as Provider>::Scope>> {
        Some(vec![
            OktaScope::Openid,
            OktaScope::Email,
            OktaScope::Profile,
        ])
    }

    fn authorization_endpoint_url(&self) -> &Url {
        &self.authorization_endpoint_url
    }
}
