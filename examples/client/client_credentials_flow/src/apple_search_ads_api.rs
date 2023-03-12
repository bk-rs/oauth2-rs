/*
YOUR_CLIENT_SECRET can generate by https://github.com/bk-rs/apple-rs/blob/main/apple-search-ads-client-secret/cli/src/bin/apple_search_ads_client_secret_gen.rs

RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_client_credentials_flow_example --bin client_credentials_flow_apple_search_ads_api -- 'YOUR_CLIENT_ID' 'YOUR_CLIENT_SECRET'
*/

use std::env;

use http_api_isahc_client::IsahcClient;
use oauth2_apple::AppleProviderForSearchAdsApi;
use oauth2_client::client_credentials_grant::Flow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let client_id = env::args().nth(1).unwrap();
    let client_secret = env::args().nth(2).unwrap();

    run(client_id, client_secret).await
}

async fn run(client_id: String, client_secret: String) -> Result<(), Box<dyn std::error::Error>> {
    let flow = Flow::new(IsahcClient::new()?);
    let provider = AppleProviderForSearchAdsApi::new(client_id, client_secret)?;

    let access_token_body = flow.execute(&provider, None).await?;

    println!("access_token_body: {access_token_body:?}");

    Ok(())
}
