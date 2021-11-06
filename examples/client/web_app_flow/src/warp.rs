/*
Read README.md
*/

use std::{error, sync::Arc};

use futures_util::future;
use log::info;
use oauth2_signin::oauth2_client::utils::gen_state;
use warp::{http::Uri, Filter};
use warp_sessions::{MemoryStore, SessionWithStore};

use oauth2_client_web_app_flow_example::{config::Config, context::Context};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();
    run(Config::new()?).await
}

async fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let ctx = Arc::new(Context::new(config)?);

    let session_store = MemoryStore::new();

    let routes = filters(ctx.clone(), session_store);
    let server_http = warp::serve(routes.clone()).run(([127, 0, 0, 1], 8080));
    let server_https = warp::serve(routes)
        .tls()
        .cert_path(&ctx.config.tls_cert_path)
        .key_path(&ctx.config.tls_key_path)
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
        .insert(state_session_key(&provider).as_str(), state.to_owned())
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
        .get::<String>(state_session_key(&provider).as_str());
    session_with_store
        .session
        .remove(state_session_key(&provider).as_str());

    let ret = flow.handle_callback(query_raw, state).await;

    info!("{} {:?}", provider, ret);

    Ok((warp::reply::html(format!("{:?}", ret)), session_with_store))
}

fn state_session_key(provider: &str) -> String {
    format!("state_{}", provider)
}
