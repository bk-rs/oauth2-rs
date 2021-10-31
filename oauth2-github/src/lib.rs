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
#[derive(Debug, Clone)]
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
    type Scope = GithubScope;

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
#[derive(Debug, Clone)]
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
    type Scope = GithubScope;

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
pub enum GithubScope {
    //
    #[serde(rename = "repo")]
    Repo,
    #[serde(rename = "repo:status")]
    RepoStatus,
    #[serde(rename = "repo_deployment")]
    RepoDeployment,
    #[serde(rename = "public_repo")]
    PublicRepo,
    #[serde(rename = "repo:invite")]
    RepoInvite,
    #[serde(rename = "security_events")]
    SecurityEvents,
    //
    #[serde(rename = "admin:repo_hook")]
    AdminRepoHook,
    #[serde(rename = "write:repo_hook")]
    WriteRepoHook,
    #[serde(rename = "read:repo_hook")]
    ReadRepoHook,
    //
    #[serde(rename = "admin:org")]
    AdminOrg,
    #[serde(rename = "write:org")]
    WriteOrg,
    #[serde(rename = "read:org")]
    ReadOrg,
    //
    #[serde(rename = "admin:public_key")]
    AdminPublicKey,
    #[serde(rename = "write:public_key")]
    WritePublicKey,
    #[serde(rename = "read:public_key")]
    ReadPublicKey,
    //
    #[serde(rename = "admin:org_hook")]
    AdminOrgHook,
    //
    #[serde(rename = "gist")]
    Gist,
    //
    #[serde(rename = "notifications")]
    Notifications,
    //
    #[serde(rename = "user")]
    User,
    #[serde(rename = "read:user")]
    ReadUser,
    #[serde(rename = "user:email")]
    UserEmail,
    #[serde(rename = "user:follow")]
    UserFollow,
    //
    #[serde(rename = "delete_repo")]
    DeleteRepo,
    //
    #[serde(rename = "write:discussion")]
    WriteDiscussion,
    #[serde(rename = "read:discussion")]
    ReadDiscussion,
    //
    #[serde(rename = "write:packages")]
    WritePackages,
    #[serde(rename = "read:packages")]
    ReadPackages,
    #[serde(rename = "delete:packages")]
    DeletePackages,
    //
    #[serde(rename = "admin:gpg_key")]
    AdminGpgKey,
    #[serde(rename = "write:gpg_key")]
    WriteGpgKey,
    #[serde(rename = "read:gpg_key")]
    ReadGpgKey,
    //
    #[serde(rename = "codespace")]
    Codespace,
    //
    #[serde(rename = "workflow")]
    Workflow,
}
impl Scope for GithubScope {}
