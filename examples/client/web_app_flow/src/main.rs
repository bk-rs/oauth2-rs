/*
cp config/clients.toml.example config/clients.toml

CAROOT=$(pwd)/mkcert mkcert -install

RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p oauth2_client_web_app_flow_example

sudo socat tcp-listen:80,reuseaddr,fork tcp:127.0.0.1:8080
sudo socat tcp-listen:443,reuseaddr,fork tcp:127.0.0.1:8443

xdg-open http://oauth2-rs.lvh.me/auth/github

xdg-open https://oauth2-rs.lvh.me/auth/google
*/

use std::{collections::HashMap, env, error, fs, path::PathBuf, sync::Arc};

use futures_util::future;
use http_api_isahc_client::IsahcClient;
use log::info;
use oauth2_github::{GithubProviderWithWebApplication, GithubScope, GithubUserInfoEndpoint};
use oauth2_google::{
    GoogleProviderForWebServerApps, GoogleProviderForWebServerAppsAccessType, GoogleScope,
    GoogleUserInfoEndpoint,
};
use oauth2_signin::{
    oauth2_client::{
        re_exports::{ClientId, ClientSecret, RedirectUri},
        utils::gen_state,
    },
    web_app::SigninFlow,
};

use serde::Deserialize;
use warp::{http::Uri, Filter};
use warp_sessions::{MemoryStore, SessionWithStore};

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

struct Context {
    signin_flow_map: HashMap<&'static str, SigninFlow<IsahcClient>>,
}

async fn run(
    clients_config: ClientsConfig,
    tls_cert_path: PathBuf,
    tls_key_path: PathBuf,
) -> Result<(), Box<dyn error::Error>> {
    let mut signin_flow_map = HashMap::new();
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

    let session_store = MemoryStore::new();

    let ctx = Arc::new(Context { signin_flow_map });

    let routes = filters(ctx, session_store);
    let server_http = warp::serve(routes.clone()).run(([127, 0, 0, 1], 8080));
    let server_https = warp::serve(routes)
        .tls()
        .cert_path(tls_cert_path)
        .key_path(tls_key_path)
        .run(([127, 0, 0, 1], 8443));

    future::join(server_http, server_https).await;

    Ok(())
}

fn filters(
    ctx: Arc<Context>,
    session_store: MemoryStore,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let ctx_t = ctx.clone();
    let session_store_t = session_store.clone();

    warp::path!("auth" / String)
        .and(warp::any().map(move || ctx_t.clone()))
        .and(warp_sessions::request::with_session(session_store_t, None))
        .and_then(auth_handler)
        .untuple_one()
        .and_then(warp_sessions::reply::with_session)
        .or(warp::path!("auth" / String / "callback")
            .and(
                warp::query::raw()
                    .map(Some)
                    .or(warp::any().map(|| None))
                    .unify(),
            )
            .and(warp::any().map(move || ctx.clone()))
            .and(warp_sessions::request::with_session(session_store, None))
            .and_then(auth_callback_handler)
            .untuple_one()
            .and_then(warp_sessions::reply::with_session))
}

async fn auth_handler(
    provider: String,
    ctx: Arc<Context>,
    mut session_with_store: SessionWithStore<MemoryStore>,
) -> Result<(impl warp::Reply, SessionWithStore<MemoryStore>), warp::Rejection> {
    let flow = ctx.signin_flow_map.get(provider.as_str()).unwrap();

    let state = gen_state();

    session_with_store
        .session
        .insert(format!("state_{}", provider).as_str(), state.to_owned())
        .unwrap();
    let url = flow.build_authorization_url(state).unwrap();

    info!("{} {:?}", provider, url.as_str());

    Ok((
        warp::redirect::temporary(url.as_str().parse::<Uri>().unwrap()),
        session_with_store,
    ))
}

async fn auth_callback_handler(
    provider: String,
    query_raw: Option<String>,
    ctx: Arc<Context>,
    mut session_with_store: SessionWithStore<MemoryStore>,
) -> Result<(impl warp::Reply, SessionWithStore<MemoryStore>), warp::Rejection> {
    let query_raw = query_raw.unwrap();

    let flow = ctx.signin_flow_map.get(provider.as_str()).unwrap();

    let state = session_with_store
        .session
        .get::<String>(format!("state_{}", provider).as_str())
        .unwrap();
    session_with_store
        .session
        .remove(format!("state_{}", provider).as_str());

    let ret = flow.handle_callback(query_raw, state).await;

    info!("{} {:?}", provider, ret);

    Ok((warp::reply::html(format!("{:?}", ret)), session_with_store))
}
