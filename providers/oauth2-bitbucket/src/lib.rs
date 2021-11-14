use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://bitbucket.org/site/oauth2/access_token";
pub const AUTHORIZATION_URL: &str = "https://bitbucket.org/site/oauth2/authorize";

pub mod authorization_code_grant;

pub use authorization_code_grant::BitbucketProviderWithWebApplication;

pub mod extensions;
pub use extensions::BitbucketExtensionsBuilder;

// Ref https://support.atlassian.com/bitbucket-cloud/docs/use-oauth-on-bitbucket-cloud/#Scopes
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum BitbucketScope {
    //
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "account:write")]
    AccountWrite,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "repository")]
    Repository,
    //
    //
    //
    // TODO
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for BitbucketScope {}
