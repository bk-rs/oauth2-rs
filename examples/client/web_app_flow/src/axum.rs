/*
Read README.md
*/

use core::convert::Infallible;
use std::{error, sync::Arc};

use axum::{
    extract::{Extension, Path, RawQuery},
    response::{Html, Redirect},
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use axum_sessions::{async_session::MemoryStore, extractors::WritableSession, SessionLayer};
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
use rand::{thread_rng, Rng};

use oauth2_client_web_app_flow_example::{config::Config, context::Context, helpers::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();
    run(Config::new()?).await
}

async fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let ctx = Arc::new(Context::new(config)?);

    //
    let session_store = MemoryStore::new();
    let session_secret = thread_rng().gen::<[u8; 128]>();
    let session_layer = SessionLayer::new(session_store, &session_secret);

    //
    let app = Router::new()
        .route("/auth/:provider", get(auth_handler))
        .route("/auth/:provider/callback", get(auth_callback_handler))
        .layer(Extension(ctx.clone()))
        .layer(session_layer.clone());

    //
    let server_http =
        axum::Server::bind(&"127.0.0.1:8080".parse()?).serve(app.clone().into_make_service());

    //
    let server_https_bind_config =
        RustlsConfig::from_pem_file(&ctx.config.tls_cert_path, &ctx.config.tls_key_path).await?;

    let server_https =
        axum_server::bind_rustls("127.0.0.1:8443".parse()?, server_https_bind_config)
            .serve(app.into_make_service());

    //
    let _ = future::join(server_http, server_https).await;

    Ok(())
}

async fn auth_handler(
    Path(provider): Path<String>,
    Extension(ctx): Extension<Arc<Context>>,
    mut session: WritableSession,
) -> Result<Redirect, Infallible> {
    let flow = ctx.signin_flow_map.get(provider.as_str()).unwrap();

    let mut config = SigninFlowBuildAuthorizationUrlConfiguration::new();

    let state = gen_state(10);
    session
        .insert(state_session_key(&provider).as_str(), state.to_owned())
        .unwrap();
    config.set_state(state);

    if flow.is_oidc_enabled() {
        let nonce = gen_nonce(32);
        session
            .insert(nonce_session_key(&provider).as_str(), nonce.to_owned())
            .unwrap();
        config.set_nonce(nonce);
    }

    if flow.is_pkce_enabled() {
        let code_verifier = gen_code_verifier(64);
        session
            .insert(
                code_verifier_session_key(&provider).as_str(),
                code_verifier.to_owned(),
            )
            .unwrap();
        let (code_challenge, code_challenge_method) = gen_code_challenge(code_verifier, None);
        config.set_code_challenge(code_challenge, code_challenge_method);
    }

    let url = flow.build_authorization_url(config).unwrap();

    info!("{} authorization_url {}", provider, url.as_str());

    Ok(Redirect::temporary(url.as_str()))
}

async fn auth_callback_handler(
    Path(provider): Path<String>,
    RawQuery(query_raw): RawQuery,
    Extension(ctx): Extension<Arc<Context>>,
    mut session: WritableSession,
) -> Result<Html<String>, Infallible> {
    let query_raw = query_raw.unwrap();

    let flow = ctx.signin_flow_map.get(provider.as_str()).unwrap();

    let mut config = SigninFlowHandleCallbackByQueryConfiguration::new();

    let state = session.get::<String>(state_session_key(&provider).as_str());
    session.remove(state_session_key(&provider).as_str());
    info!("{} state {:?}", provider, state);
    if let Some(state) = state {
        config.set_state(state);
    }

    if flow.is_oidc_enabled() {
        let nonce = session.get::<String>(nonce_session_key(&provider).as_str());
        session.remove(nonce_session_key(&provider).as_str());
        info!("{} nonce {:?}", provider, nonce);
        if let Some(nonce) = nonce {
            config.set_nonce(nonce);
        }
    }

    if flow.is_pkce_enabled() {
        let code_verifier = session.get::<String>(code_verifier_session_key(&provider).as_str());
        session.remove(code_verifier_session_key(&provider).as_str());
        info!("{} code_verifier {:?}", provider, code_verifier);
        if let Some(code_verifier) = code_verifier {
            config.set_code_verifier(code_verifier);
        }
    }

    let ret = flow.handle_callback_by_query(query_raw, config).await;

    info!("{} {:?}", provider, ret);

    Ok(Html(format!("{:?}", ret)))
}
