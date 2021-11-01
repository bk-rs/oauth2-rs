use oauth2_core::{
    provider::{Map, Url, UrlParseError, Value},
    types::{ClientId, ClientSecret, Scope},
    Provider, ProviderExtDeviceAuthorizationGrant,
};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
pub const AUTHORIZATION_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const DEVICE_AUTHORIZATION_URL: &str = "https://oauth2.googleapis.com/device/code";

//
//
//
#[derive(Debug, Clone)]
pub struct GoogleProviderForTvAndDeviceApps {
    client_id: ClientId,
    client_secret: ClientSecret,
    //
    token_endpoint_url: Url,
    device_authorization_endpoint_url: Url,
}
impl GoogleProviderForTvAndDeviceApps {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Result<Self, UrlParseError> {
        Ok(Self {
            client_id,
            client_secret,
            token_endpoint_url: TOKEN_URL.parse()?,
            device_authorization_endpoint_url: DEVICE_AUTHORIZATION_URL.parse()?,
        })
    }
}
impl Provider for GoogleProviderForTvAndDeviceApps {
    type Scope = GoogleScope;

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
impl ProviderExtDeviceAuthorizationGrant for GoogleProviderForTvAndDeviceApps {
    fn device_authorization_endpoint_url(&self) -> Url {
        self.device_authorization_endpoint_url.to_owned()
    }

    fn device_access_token_request_body_extensions(&self) -> Option<Map<String, Value>> {
        let mut map = Map::new();
        map.insert(
            "client_secret".to_owned(),
            Value::String(self.client_secret.to_owned()),
        );
        Some(map)
    }
}

//
//
//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum GoogleScope {
    //
    #[serde(rename = "email")]
    #[serde(alias = "https://www.googleapis.com/auth/userinfo.email")]
    Email,
    #[serde(rename = "profile")]
    #[serde(alias = "https://www.googleapis.com/auth/userinfo.profile")]
    Profile,
    //
    #[serde(rename = "openid")]
    Openid,
    //
    #[serde(rename = "https://www.googleapis.com/auth/drive.file")]
    DriveFile,
    //
    #[serde(rename = "https://www.googleapis.com/auth/youtube")]
    Youtube,
    #[serde(rename = "https://www.googleapis.com/auth/youtube.readonly")]
    YoutubeReadonly,
    //
    // TODO
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for GoogleScope {}
