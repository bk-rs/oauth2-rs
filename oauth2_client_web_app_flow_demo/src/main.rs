/*
cp config/clients.toml.example config/clients.toml

CAROOT=$(pwd)/mkcert mkcert -install

RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p oauth2_client_web_app_flow_demo

open http://oauth2-rs.lvh.me/auth/github

open https://oauth2-rs.lvh.me/auth/google
*/

use std::{collections::HashMap, env, error, fs, path::PathBuf, sync::Arc};

use futures_util::future;
use http_api_isahc_client::IsahcClient;
use oauth2_github::{GithubProviderWithWebApplication, GithubScope, GithubUserInfoEndpoint};
use oauth2_google::{
    GoogleProviderForWebServerApps, GoogleProviderForWebServerAppsAccessType, GoogleScope,
    GoogleUserInfoEndpoint,
};
use oauth2_signin::{
    oauth2_client::re_exports::{ClientId, ClientSecret, RedirectUri},
    web_app::{SigninFlow, SigninFlowMap},
};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

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

    run(clients_config, tls_cert_path, tls_key_path).await
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

async fn run(
    clients_config: ClientsConfig,
    tls_cert_path: PathBuf,
    tls_key_path: PathBuf,
) -> Result<(), Box<dyn error::Error>> {
    let mut signin_flow_map = SigninFlowMap::new();
    signin_flow_map.insert(
        "github",
        SigninFlow::new(
            IsahcClient::new()?,
            GithubProviderWithWebApplication::new(
                clients_config.github.client_id.to_owned(),
                clients_config.github.client_secret.to_owned(),
                clients_config.github.redirect_uri.to_owned(),
            )?,
            vec![GithubScope::PublicRepo, GithubScope::UserEmail],
            GithubUserInfoEndpoint,
        ),
    );
    signin_flow_map.insert(
        "google",
        SigninFlow::new(
            IsahcClient::new()?,
            GoogleProviderForWebServerApps::new(
                clients_config.google.client_id.to_owned(),
                clients_config.google.client_secret.to_owned(),
                clients_config.google.redirect_uri.to_owned(),
            )?
            .configure(|mut x| {
                x.access_type = Some(GoogleProviderForWebServerAppsAccessType::Offline);
                x.include_granted_scopes = Some(true);
            }),
            vec![GoogleScope::Email, GoogleScope::DriveFile],
            GoogleUserInfoEndpoint,
        ),
    );

    let ctx = Arc::new(Context { signin_flow_map });

    let routes = filters::filters(ctx.clone());
    let server_http = warp::serve(routes.clone()).run(([127, 0, 0, 1], 80));
    let server_https = warp::serve(routes)
        .tls()
        .cert_path(tls_cert_path)
        .key_path(tls_key_path)
        .run(([127, 0, 0, 1], 443));

    future::join(server_http, server_https).await;

    Ok(())
}

pub struct Context {
    pub signin_flow_map: SigninFlowMap<IsahcClient>,
}

pub mod filters {
    use super::Context;

    use std::sync::Arc;

    use log::info;
    use warp::{http::Uri, Filter};

    pub fn filters(
        ctx: Arc<Context>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let ctx_t = ctx.clone();

        warp::path!(String)
            .and(warp::any().map(move || ctx_t.clone()))
            .map(|x: String, ctx: Arc<Context>| Ok(warp::reply::html("")));

        warp::path!("auth" / String)
            .and(warp::any().map(move || ctx_t.clone()))
            .and_then(auth_handler)
            .or(warp::path!("auth" / String / "callback")
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
        provider: String,
        ctx: Arc<Context>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let flow = ctx.signin_flow_map.get(provider.as_str()).unwrap();

        let state = "TODO".to_owned();

        let url = flow.build_authorization_url(state).unwrap();

        info!("{} {:?}", provider, url.as_str());

        Ok(warp::redirect::temporary(
            url.as_str().parse::<Uri>().unwrap(),
        ))
    }

    async fn auth_callback_handler(
        provider: String,
        query_raw: Option<String>,
        ctx: Arc<Context>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let query_raw = query_raw.unwrap();

        let flow = ctx.signin_flow_map.get(provider.as_str()).unwrap();

        let state = "TODO".to_owned();

        let ret = flow.handle_callback(query_raw, state).await;

        info!("{} {:?}", provider, ret);

        Ok(warp::reply::html(format!("{:?}", ret)))
    }
}
