/*
RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_device_flow_example --bin device_flow_google -- 'YOUR_CLIENT_ID' 'YOUR_CLIENT_SECRET'
*/

use std::{env, error};

use http_api_isahc_client::IsahcClient;
use oauth2_client::device_authorization_grant::Flow;
use oauth2_google::{GoogleProviderForTvAndDeviceApps, GoogleScope};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let client_id = env::args().nth(1).unwrap();
    let client_secret = env::args().nth(2).unwrap();

    run(client_id, client_secret).await
}

async fn run(client_id: String, client_secret: String) -> Result<(), Box<dyn error::Error>> {
    let scopes = vec![GoogleScope::Email, GoogleScope::DriveFile];

    let flow = Flow::new(IsahcClient::new()?, IsahcClient::new()?);
    let provider = GoogleProviderForTvAndDeviceApps::new(client_id, client_secret)?;

    let access_token_body = flow
        .execute(
            &provider,
            scopes,
            |user_code, verification_uri, _verification_uri_complete| {
                println!("open [{verification_uri}] then input [{user_code}]");
            },
        )
        .await?;

    println!("access_token_body: {access_token_body:?}");

    Ok(())
}
