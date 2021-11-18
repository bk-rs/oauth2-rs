/*
RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_desktop_app_flow_example --bin desktop_app_flow_mastodon_social -- 'YOUR_CLIENT_ID' 'YOUR_CLIENT_SECRET'
*/

use std::{env, error, io, thread};

use http_api_isahc_client::IsahcClient;
use oauth2_client::{authorization_code_grant::Flow, oauth2_core::types::RedirectUri};
use oauth2_mastodon::{MastodonProviderForEndUsers, MastodonScope, BASE_URL_MASTODON_SOCIAL};
use web_view::{Content, WebViewBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let client_id = env::args().nth(1).unwrap();
    let client_secret = env::args().nth(2).unwrap();

    run(client_id, client_secret).await
}

async fn run(client_id: String, client_secret: String) -> Result<(), Box<dyn error::Error>> {
    let scopes = vec![MastodonScope::Read, MastodonScope::Write];

    let flow = Flow::new(IsahcClient::new()?);
    let provider = MastodonProviderForEndUsers::new(
        BASE_URL_MASTODON_SOCIAL,
        client_id,
        client_secret,
        RedirectUri::Oob,
    )?;

    let authorization_url = flow.build_authorization_url(&provider, scopes, None)?;

    println!("authorization_url: {:?}", authorization_url.as_str());

    thread::spawn(move || {
        WebViewBuilder::new()
            .title("OAuth2")
            .content(Content::Url(authorization_url.as_str()))
            .size(800, 600)
            .resizable(true)
            .debug(false)
            .user_data(())
            .invoke_handler(|_webview, _arg| Ok(()))
            .build()
            .unwrap()
            .run()
            .unwrap();
    })
    .join()
    .unwrap();

    println!("Enter code: ");
    let mut code = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut code)?;
    code = code.trim_end().to_owned();
    println!("code: {:?}", code);

    let access_token_body = flow.handle_callback(&provider, code, None).await?;

    println!("access_token_body: {:?}", access_token_body);

    Ok(())
}
