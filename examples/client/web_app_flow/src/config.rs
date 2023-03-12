use std::{env, error, fs, path::PathBuf};

use serde::Deserialize;
use serde_json::{Map, Value};

use oauth2_signin::oauth2_client::re_exports::{ClientId, ClientSecret, RedirectUri};

#[derive(Debug, Clone)]
pub struct Config {
    pub clients_config: ClientsConfig,
    pub tls_cert_path: PathBuf,
    pub tls_key_path: PathBuf,
}
impl Config {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let manifest_path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            PathBuf::from(&manifest_dir)
        } else {
            PathBuf::new()
        };
        let clients_config_path = manifest_path.join("config/clients.toml");
        let tls_cert_path = manifest_path.join("mkcert/oauth2-rs.lvh.me.pem");
        let tls_key_path = manifest_path.join("mkcert/oauth2-rs.lvh.me-key.pem");

        let clients_config_str = fs::read_to_string(clients_config_path)?;
        let clients_config: ClientsConfig = toml::from_str(&clients_config_str)?;

        Ok(Self {
            clients_config,
            tls_cert_path,
            tls_key_path,
        })
    }
}

//
#[derive(Deserialize, Debug, Clone)]
pub struct ClientsConfig {
    pub github: ClientConfig,
    pub google: ClientConfig,
    pub twitch: ClientConfig,
    pub mastodon_social: ClientConfig,
    pub apple: ClientConfig,
    pub instagram: ClientConfig,
    pub facebook: ClientConfig,
    pub amazon: ClientConfig,
    pub gitlab: ClientConfig,
    pub bitbucket: ClientConfig,
    pub digitalocean: ClientConfig,
    pub dropbox: ClientConfig,
    pub linkedin: ClientConfig,
    pub microsoft: ClientConfig,
    pub yahoo: ClientConfig,
    pub okta: ClientConfig,
    pub pinterest: ClientConfig,
    pub baidu: ClientConfig,
    pub twitter: ClientConfig,
    pub tiktok: ClientConfig,
    pub zoho: ClientConfig,
    pub linode: ClientConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub redirect_uri: RedirectUri,
    //
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
