use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://login.linode.com/oauth/token";
pub const AUTHORIZATION_URL: &str = "https://login.linode.com/oauth/authorize";

pub mod authorization_code_grant;

pub use authorization_code_grant::LinodeProviderWithWebApplication;

pub mod extensions;
pub use extensions::LinodeExtensionsBuilder;

// Ref https://www.linode.com/docs/api/#oauth-reference
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum LinodeScope {
    //
    #[serde(rename = "account:read_only")]
    AccountReadOnly,
    #[serde(rename = "account:read_write")]
    AccountReadWrite,
    #[serde(rename = "linodes:read_only")]
    LinodesReadOnly,
    #[serde(rename = "linodes:read_write")]
    LinodesReadWrite,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for LinodeScope {}
