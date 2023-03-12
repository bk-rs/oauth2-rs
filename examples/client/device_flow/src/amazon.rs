/*
RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_device_flow_example --bin device_flow_amazon -- 'YOUR_CLIENT_ID'
*/

use std::env;

use http_api_isahc_client::IsahcClient;
use oauth2_amazon::{AmazonProviderWithDevices, AmazonScope};
use oauth2_client::device_authorization_grant::Flow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let client_id = env::args().nth(1).unwrap();

    run(client_id).await
}

async fn run(client_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let scopes = vec![AmazonScope::Profile, AmazonScope::PostalCode];

    let flow = Flow::new(IsahcClient::new()?, IsahcClient::new()?);
    let provider = AmazonProviderWithDevices::new(client_id)?;

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
