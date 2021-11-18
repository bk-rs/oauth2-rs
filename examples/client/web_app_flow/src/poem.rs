/*
Read README.md
*/

use std::{error, fs, sync::Arc};

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
use poem::{
    get, handler,
    http::Uri,
    listener::{Listener, TcpListener, TlsConfig},
    session::{CookieConfig, CookieSession, Session},
    web::{Data, Path, Redirect},
    EndpointExt, Error as PoemError, FromRequest, Request, RequestBody, Route, Server,
};

use oauth2_client_web_app_flow_example::{config::Config, context::Context};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();
    run(Config::new()?).await
}

async fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let ctx = Arc::new(Context::new(config)?);

    let app = Route::new()
        .at("/auth/:provider", get(auth_handler))
        .at("/auth/:provider/callback", get(auth_callback_handler))
        .data(ctx.clone())
        .with(CookieSession::new(CookieConfig::default().secure(false)));

    let app_t = Route::new()
        .at("/auth/:provider", get(auth_handler))
        .at("/auth/:provider/callback", get(auth_callback_handler))
        .data(ctx.clone())
        .with(CookieSession::new(CookieConfig::default().secure(false)));

    let server_http = Server::new(TcpListener::bind("127.0.0.1:8080"))
        .await?
        .run(app);
    let server_https = Server::new(
        TcpListener::bind("127.0.0.1:8443").tls(
            TlsConfig::new()
                .key(fs::read_to_string(&ctx.config.tls_key_path)?)
                .cert(fs::read_to_string(&ctx.config.tls_cert_path)?),
        ),
    )
    .await?
    .run(app_t);

    let _ = future::join(server_http, server_https).await;

    Ok(())
}

#[handler]
async fn auth_handler(
    Path(provider): Path<String>,
    ctx: Data<&Arc<Context>>,
    session: &Session,
) -> Result<Redirect, PoemError> {
    let flow = ctx
        .signin_flow_map
        .get(provider.as_str())
        .ok_or("provider not found")?;

    let mut config = SigninFlowBuildAuthorizationUrlConfiguration::new();

    let state = gen_state(10);
    session.set(state_session_key(&provider).as_str(), state.to_owned());
    config.set_state(state);

    if flow.is_oidc_enabled() {
        let nonce = gen_nonce(32);
        session.set(nonce_session_key(&provider).as_str(), nonce.to_owned());
        config.set_nonce(nonce);
    };

    if flow.is_pkce_enabled() {
        let code_verifier = gen_code_verifier(64);
        session.set(
            code_verifier_session_key(&provider).as_str(),
            code_verifier.to_owned(),
        );
        let (code_challenge, code_challenge_method) = gen_code_challenge(code_verifier, None);
        config.set_code_challenge(code_challenge, code_challenge_method);
    }

    let url = flow.build_authorization_url(config)?;

    info!("{} authorization_url {}", provider, url.as_str());

    Ok(Redirect::temporary(url.as_str().parse::<Uri>()?))
}

struct QueryRaw(String);

#[async_trait::async_trait]
impl<'a> FromRequest<'a> for QueryRaw {
    type Error = String;

    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self, Self::Error> {
        Ok(Self(req.uri().query().unwrap_or_default().to_owned()))
    }
}

#[handler]
async fn auth_callback_handler(
    Path(provider): Path<String>,
    QueryRaw(query_raw): QueryRaw,
    ctx: Data<&Arc<Context>>,
    session: &Session,
) -> Result<String, PoemError> {
    let flow = ctx
        .signin_flow_map
        .get(provider.as_str())
        .ok_or("provider not found")?;

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

    Ok(format!("{:?}", ret))
}

fn state_session_key(provider: &str) -> String {
    format!("state_{}", provider)
}

fn nonce_session_key(provider: &str) -> String {
    format!("nonce_{}", provider)
}

fn code_verifier_session_key(provider: &str) -> String {
    format!("code_verifier_{}", provider)
}
