/*
cp clients.toml.example clients.toml

RUST_BACKTRACE=1 RUST_LOG=debug,isahc=off cargo run -p oauth2_client_web_app_flow_demo
*/

use std::{collections::HashMap, env, error, fs, path::PathBuf};

use http_api_isahc_client::IsahcClient;
use oauth2_client::{
    authorization_code_grant::Flow,
    provider::{ClientId, ClientSecret, RedirectUri},
};
use oauth2_github::{GithubProviderWithWebApplication, GithubScope};
use oauth2_google::{GoogleProviderForWebServerApps, GoogleScope};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let manifest_path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(&manifest_dir)
    } else {
        PathBuf::new()
    };
    let clients_config_file = manifest_path.join("clients.toml");

    let clients_config_str = fs::read_to_string(clients_config_file)?;
    let clients_config: ClientsConfig = toml::from_str(&clients_config_str)?;

    run(clients_config).await
}

#[derive(Deserialize, Debug)]
struct ClientsConfig {
    github: ClientConfig,
    google: ClientConfig,
}

#[derive(Deserialize, Debug)]
struct ClientConfig {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUri,
}

async fn run(clients_config: ClientsConfig) -> Result<(), Box<dyn error::Error>> {
    let client = IsahcClient::new()?;
    let flow = Flow::new(client);

    let provider_map: HashMap<ProviderKey, ProviderValue> = vec![
        (
            ProviderKey::Github,
            ProviderValue::Github((
                GithubProviderWithWebApplication::new(
                    clients_config.github.client_id.to_owned(),
                    clients_config.github.client_secret.to_owned(),
                    clients_config.github.redirect_uri.to_owned(),
                )?,
                vec![GithubScope::PublicRepo, GithubScope::UserEmail],
            )),
        ),
        (
            ProviderKey::Google,
            ProviderValue::Google((
                GoogleProviderForWebServerApps::new(
                    clients_config.google.client_id.to_owned(),
                    clients_config.google.client_secret.to_owned(),
                    clients_config.google.redirect_uri.to_owned(),
                )?,
                vec![GoogleScope::Email, GoogleScope::DriveFile],
            )),
        ),
    ]
    .into_iter()
    .collect();

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ProviderKey {
    Github,
    Google,
}
#[derive(Debug, Clone)]
enum ProviderValue {
    Github((GithubProviderWithWebApplication, Vec<GithubScope>)),
    Google((GoogleProviderForWebServerApps, Vec<GoogleScope>)),
}
