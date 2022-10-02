/*
RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_device_flow_example --bin device_flow_baidu -- 'YOUR_APP_KEY' 'YOUR_SECRET_KEY'
*/

use std::{env, error};

use http_api_isahc_client::IsahcClient;
use oauth2_baidu::{BaiduProviderWithDevice, BaiduScope};
use oauth2_client::device_authorization_grant::Flow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let app_key = env::args().nth(1).unwrap();
    let secret_key = env::args().nth(2).unwrap();

    run(app_key, secret_key).await
}

async fn run(app_key: String, secret_key: String) -> Result<(), Box<dyn error::Error>> {
    let scopes = vec![BaiduScope::Basic, BaiduScope::Netdisk];

    let flow = Flow::new(IsahcClient::new()?, IsahcClient::new()?);
    let provider = BaiduProviderWithDevice::new(app_key, secret_key)?;

    let access_token_body = flow
        .execute(
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
