/*
RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_client_credentials_flow_example --bin client_credentials_flow_mastodon_social -- 'YOUR_CLIENT_ID' 'YOUR_CLIENT_SECRET'
*/

use std::{env, error};

use http_api_isahc_client::IsahcClient;
use oauth2_client::client_credentials_grant::Flow;
use oauth2_mastodon::{MastodonProviderForApplications, MastodonScope, BASE_URL_MASTODON_SOCIAL};

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
    let provider =
        MastodonProviderForApplications::new(BASE_URL_MASTODON_SOCIAL, client_id, client_secret)?;

    let access_token_body = flow.execute(&provider, scopes).await?;

    println!("access_token_body: {access_token_body:?}");

    Ok(())
}
