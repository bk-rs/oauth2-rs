use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://www.dropbox.com/oauth2/token";
pub const AUTHORIZATION_URL: &str = "https://www.dropbox.com/oauth2/authorize";

pub mod authorization_code_grant;

pub use authorization_code_grant::DropboxProviderWithWebApplication;

pub mod extensions;
pub use extensions::DropboxExtensionsBuilder;

// Ref App Console, Permissions tab
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum DropboxScope {
    //
    #[serde(rename = "account_info.write")]
    AccountInfoWrite,
    #[serde(rename = "account_info.read")]
    AccountInfoRead,
    #[serde(rename = "files.metadata.write")]
    FilesMetadataWrite,
    #[serde(rename = "files.metadata.read")]
    FilesMetadataRead,
    #[serde(rename = "files.content.write")]
    FilesContentWrite,
    #[serde(rename = "files.content.read")]
    FilesContentRead,
    #[serde(rename = "sharing.write")]
    SharingWrite,
    #[serde(rename = "sharing.read")]
    SharingRead,
    #[serde(rename = "file_requests.write")]
    FileRequestsWrite,
    #[serde(rename = "file_requests.read")]
    FileRequestsRead,
    #[serde(rename = "contacts.write")]
    ContactsWrite,
    #[serde(rename = "contacts.read")]
    ContactsRead,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for DropboxScope {}
