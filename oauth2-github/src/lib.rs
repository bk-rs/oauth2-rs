use oauth2_core::{
    types::{ClientId, ClientSecret, Scope},
    url::{ParseError as UrlParseError, Url},
    Provider, ProviderExtAuthorizationCodeGrant, ProviderExtDeviceAuthorizationGrant,
};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
pub const AUTHORIZATION_URL: &str = "https://github.com/login/oauth/authorize";
pub const DEVICE_AUTHORIZATION_URL: &str = "https://github.com/login/device/code";

//
//
//
pub struct GithubProviderWithWebApplication {
    client_id: ClientId,
    client_secret: ClientSecret,
    //
    token_endpoint_url: Url,
    authorization_endpoint_url: Url,
}
impl GithubProviderWithWebApplication {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            token_endpoint_url: TOKEN_URL.parse()?,
            authorization_endpoint_url: AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for GithubProviderWithWebApplication {
    type Scope = GithubOauthScope;

    fn client_id(&self) -> Option<ClientId> {
        Some(self.client_id.to_owned())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        Some(self.client_secret.to_owned())
    }

    fn token_endpoint_url(&self) -> Url {
        self.token_endpoint_url.to_owned()
    }
}
impl ProviderExtAuthorizationCodeGrant for GithubProviderWithWebApplication {
    fn authorization_endpoint_url(&self) -> Url {
        self.authorization_endpoint_url.to_owned()
    }
}

//
//
//
pub struct GithubProviderWithDevice {
    client_id: ClientId,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl GithubProviderWithDevice {
    pub fn new(client_id: ClientId) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            token_endpoint_url: TOKEN_URL.parse()?,
            device_authorization_endpoint_url: DEVICE_AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for GithubProviderWithDevice {
    type Scope = GithubOauthScope;

    fn client_id(&self) -> Option<ClientId> {
        Some(self.client_id.to_owned())
    }

    fn client_secret(&self) -> Option<ClientSecret> {
        None
    }

    fn token_endpoint_url(&self) -> Url {
        self.token_endpoint_url.to_owned()
    }
}
impl ProviderExtDeviceAuthorizationGrant for GithubProviderWithDevice {
    fn device_authorization_endpoint_url(&self) -> Url {
        self.device_authorization_endpoint_url.to_owned()
    }
}

//
//
//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum GithubOauthScope {
    #[serde(rename = "repo")]
    Repo,
    #[serde(rename = "repo:status")]
    RepoStatus,
    // TODO
}
impl Scope for GithubOauthScope {}
