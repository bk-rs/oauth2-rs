/*
Read README.md
*/

use std::sync::Arc;

use futures_util::future;
use log::info;
use oauth2_signin::{
    oauth2_client::{
        oauth2_core::utils::gen_code_challenge,
        utils::{gen_code_verifier, gen_nonce, gen_state},
    },
    web_app::{
        SigninFlowBuildAuthorizationUrlConfiguration, SigninFlowHandleCallbackByQueryConfiguration,
    },
};
use warp::{http::Uri, Filter};
use warp_sessions::{MemoryStore, SessionWithStore};

use oauth2_client_web_app_flow_example::{config::Config, context::Context, helpers::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    run(Config::new()?).await
}

async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
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
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
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

    let mut config = SigninFlowBuildAuthorizationUrlConfiguration::new();

    let state = gen_state(10);
    session_with_store
        .session
        .insert(state_session_key(&provider).as_str(), state.to_owned())
        .unwrap();
    config.set_state(state);

    if flow.is_oidc_enabled() {
        let nonce = gen_nonce(32);
        session_with_store
            .session
            .insert(nonce_session_key(&provider).as_str(), nonce.to_owned())
            .unwrap();
        config.set_nonce(nonce);
    }

    if flow.is_pkce_enabled() {
        let code_verifier = gen_code_verifier(64);
        session_with_store
            .session
            .insert(
                code_verifier_session_key(&provider).as_str(),
                code_verifier.to_owned(),
            )
            .unwrap();
        let (code_challenge, code_challenge_method) = gen_code_challenge(code_verifier, None);
        config.set_code_challenge(code_challenge, code_challenge_method);
    }

    let url = flow.build_authorization_url(config).unwrap();

    info!("{provider} authorization_url {}", url.as_str());

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

    let mut config = SigninFlowHandleCallbackByQueryConfiguration::new();

    let state = session_with_store
        .session
        .get::<String>(state_session_key(&provider).as_str());
    session_with_store
        .session
        .remove(state_session_key(&provider).as_str());
    info!("{provider} state {state:?}");
    if let Some(state) = state {
        config.set_state(state);
    }

    if flow.is_oidc_enabled() {
        let nonce = session_with_store
            .session
            .get::<String>(nonce_session_key(&provider).as_str());
        session_with_store
            .session
            .remove(nonce_session_key(&provider).as_str());
        info!("{provider} nonce {nonce:?}");
        if let Some(nonce) = nonce {
            config.set_nonce(nonce);
        }
    }

    if flow.is_pkce_enabled() {
        let code_verifier = session_with_store
            .session
            .get::<String>(code_verifier_session_key(&provider).as_str());
        session_with_store
            .session
            .remove(code_verifier_session_key(&provider).as_str());
        info!("{provider} code_verifier {code_verifier:?}");
        if let Some(code_verifier) = code_verifier {
            config.set_code_verifier(code_verifier);
        }
    }

    let ret = flow.handle_callback_by_query(query_raw, config).await;

    info!("{provider} {ret:?}");

    Ok((warp::reply::html(format!("{ret:?}")), session_with_store))
}
