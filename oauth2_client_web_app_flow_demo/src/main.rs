/*
cp clients.toml.example clients.toml

RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p oauth2_client_web_app_flow_demo

open http://oauth2-lite.lvh.me/auth/github
*/

use std::{collections::HashMap, env, error, fs, path::PathBuf, sync::Arc};

use http_api_isahc_client::IsahcClient;
use oauth2_client::{
    authorization_code_grant::Flow,
    provider::{serde_enum_str::Deserialize_enum_str, ClientId, ClientSecret, RedirectUri},
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
    let provider_map: HashMap<ProviderKey, ProviderValue> = vec![
        (
            ProviderKey::Github,
            ProviderValue::Github((
                Flow::new(IsahcClient::new()?),
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
                Flow::new(IsahcClient::new()?),
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

    let ctx = Arc::new(Context { provider_map });

    let routes = filters::filters(ctx.clone());
    warp::serve(routes).run(([127, 0, 0, 1], 80)).await;

    Ok(())
}

pub struct Context {
    pub provider_map: HashMap<ProviderKey, ProviderValue>,
}

#[derive(Deserialize_enum_str, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProviderKey {
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "google")]
    Google,
}

#[derive(Debug, Clone)]
pub enum ProviderValue {
    Github(
        (
            Flow<IsahcClient>,
            GithubProviderWithWebApplication,
            Vec<GithubScope>,
        ),
    ),
    Google(
        (
            Flow<IsahcClient>,
            GoogleProviderForWebServerApps,
            Vec<GoogleScope>,
        ),
    ),
}

pub mod filters {
    use super::{Context, ProviderKey};

    use std::sync::Arc;

    use log::info;
    use warp::{http::Uri, Filter};

    pub fn filters(
        ctx: Arc<Context>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let ctx_t = ctx.clone();

        warp::path!("auth" / ProviderKey)
            .and(warp::any().map(move || ctx_t.clone()))
            .and_then(auth_handler)
            .or(warp::path!("auth" / ProviderKey / "callback")
                .and(
                    warp::query::raw()
                        .map(Some)
                        .or(warp::any().map(|| None))
                        .unify(),
                )
                .and(warp::any().map(move || ctx.clone()))
                .and_then(auth_callback_handler))
    }

    async fn auth_handler(
        provider_key: ProviderKey,
        ctx: Arc<Context>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let provider_value = ctx.provider_map.get(&provider_key).unwrap();

        let state = "TODO".to_owned();

        let url = match provider_value {
            crate::ProviderValue::Github((flow, provider, scopes)) => flow
                .build_authorization_url(provider, scopes.to_owned(), state)
                .unwrap(),
            crate::ProviderValue::Google((flow, provider, scopes)) => flow
                .build_authorization_url(provider, scopes.to_owned(), state)
                .unwrap(),
        };

        Ok(warp::redirect::temporary(
            url.as_str().parse::<Uri>().unwrap(),
        ))
    }

    async fn auth_callback_handler(
        provider_key: ProviderKey,
        query_raw: Option<String>,
        ctx: Arc<Context>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let query_raw = query_raw.unwrap();

        let provider_value = ctx.provider_map.get(&provider_key).unwrap();

        let state = "TODO".to_owned();

        match provider_value {
            crate::ProviderValue::Github((flow, provider, _scopes)) => {
                let access_token_body = flow
                    .handle_callback(provider, query_raw, state)
                    .await
                    .unwrap();

                info!("{:?} {:?}", provider_key, access_token_body);

                Ok(warp::reply::html(format!("{:?}", access_token_body)))
            }
            crate::ProviderValue::Google((flow, provider, _scopes)) => {
                let access_token_body = flow
                    .handle_callback(provider, query_raw, state)
                    .await
                    .unwrap();

                info!("{:?} {:?}", provider_key, access_token_body);

                Ok(warp::reply::html(format!("{:?}", access_token_body)))
            }
        }
    }
}
