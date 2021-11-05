use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
pub const AUTHORIZATION_URL: &str = "https://github.com/login/oauth/authorize";
pub const DEVICE_AUTHORIZATION_URL: &str = "https://github.com/login/device/code";

pub mod authorization_code_grant;
pub mod device_authorization_grant;

pub use authorization_code_grant::GithubProviderWithWebApplication;
pub use device_authorization_grant::GithubProviderWithDevice;

pub mod additional_endpoints;
pub use additional_endpoints::GithubUserInfoEndpoint;

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
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for GithubScope {}
