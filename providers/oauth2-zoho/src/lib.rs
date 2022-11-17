use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://accounts.zoho.com/oauth/v2/token";
pub const AUTHORIZATION_URL: &str = "https://accounts.zoho.com/oauth/v2/auth";

pub mod authorization_code_grant;

pub use authorization_code_grant::{
    ZohoProviderForWebServerApps, ZohoProviderForWebServerAppsAccessType,
};

/// [Ref](https://www.zoho.com/accounts/protocol/oauth-terminology.html)
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum ZohoScope {
    /*
    https://www.site24x7.com/help/api/#authentication
    */
    #[serde(rename = "Site24x7.Account.All")]
    Site24x7AccountAll,
    #[serde(rename = "Site24x7.Account.Read")]
    Site24x7AccountRead,
    #[serde(rename = "Site24x7.Admin.All")]
    Site24x7AdminAll,
    #[serde(rename = "Site24x7.Admin.Read")]
    Site24x7AdminRead,
    #[serde(rename = "Site24x7.Reports.All")]
    Site24x7ReportsAll,
    #[serde(rename = "Site24x7.Reports.Read")]
    Site24x7ReportsRead,
    /*
    https://www.zoho.com/crm/developer/docs/api/v3/scopes.html
    */
    #[serde(rename = "ZohoCRM.users.ALL")]
    ZohoCRMUsersALL,
    #[serde(rename = "ZohoCRM.org.ALL")]
    ZohoCRMOrgALL,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for ZohoScope {}
