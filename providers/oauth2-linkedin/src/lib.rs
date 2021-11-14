use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://www.linkedin.com/oauth/v2/accessToken";
pub const AUTHORIZATION_URL: &str = "https://www.linkedin.com/oauth/v2/authorization";

pub mod authorization_code_grant;

pub use authorization_code_grant::LinkedinProviderWithWebApplication;

pub mod extensions;
pub use extensions::LinkedinExtensionsBuilder;

// Ref https://docs.microsoft.com/en-us/linkedin/shared/authentication/authentication?context=linkedin/context#permission-types
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum LinkedinScope {
    //
    #[serde(rename = "r_liteprofile")]
    ReadLiteprofile,
    #[serde(rename = "r_emailaddress")]
    ReadEmailaddress,
    #[serde(rename = "w_member_social")]
    WriteMemberSocial,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for LinkedinScope {}
