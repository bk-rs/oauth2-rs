/*
RUST_BACKTRACE=1 RUST_LOG=debug,isahc=off cargo run -p oauth2_client_device_flow_demo --bin google -- 'YOUR_CLIENT_ID' 'YOUR_CLIENT_SECRET'
*/

use std::{env, error};

use http_api_isahc_client::IsahcClient;
use oauth2_client::device_authorization_grant::Flow;
use oauth2_google::{GoogleProviderForTvAndDeviceApps, GoogleScope};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let client_id = env::args().nth(1).unwrap();
    let client_secret = env::args().nth(2).unwrap();
    let scopes = vec![GoogleScope::Email, GoogleScope::DriveFile];

    let client = IsahcClient::new()?;
    let flow = Flow::new(client);
    let provider = GoogleProviderForTvAndDeviceApps::new(client_id, client_secret)?;

    let access_token_body = flow
        .start(
            &provider,
            scopes,
            |user_code, verification_uri, _verification_uri_complete| {
                println!("open [{}] then input [{}]", verification_uri, user_code);
            },
        )
        .await?;

    println!("access_token_body: {:?}", access_token_body);

    Ok(())
}
