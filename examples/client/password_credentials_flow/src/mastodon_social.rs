/*
RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_password_credentials_flow_example --bin password_credentials_flow_mastodon_social -- 'YOUR_CLIENT_ID' 'YOUR_CLIENT_SECRET' 'YOUR_EMAIL' 'YOUR_PASSWORD'
*/

use std::{env, error};

use http_api_isahc_client::IsahcClient;
use oauth2_client::resource_owner_password_credentials_grant::Flow;
use oauth2_mastodon::{MastodonProviderForBots, MastodonScope, BASE_URL_MASTODON_SOCIAL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let client_id = env::args().nth(1).unwrap();
    let client_secret = env::args().nth(2).unwrap();
    let username = env::args().nth(3).unwrap();
    let password = env::args().nth(4).unwrap();

    run(client_id, client_secret, username, password).await
}

async fn run(
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
) -> Result<(), Box<dyn error::Error>> {
    let scopes = vec![MastodonScope::Read, MastodonScope::Write];

    let flow = Flow::new(IsahcClient::new()?);
    let provider =
        MastodonProviderForBots::new(BASE_URL_MASTODON_SOCIAL, client_id, client_secret)?;

    let access_token_body = flow.execute(&provider, scopes, username, password).await?;

    println!("access_token_body: {access_token_body:?}");

    Ok(())
}
