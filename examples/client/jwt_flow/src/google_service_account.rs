/*
RUST_BACKTRACE=1 RUST_LOG=debug cargo run -p oauth2_client_jwt_flow_example --bin jwt_flow_google_service_account -- '/path/project-123456-123456789012.json'
*/

use std::{env, fs, path::PathBuf};

use google_service_account_oauth_jwt_assertion::create_from_service_account_json_key;
use http_api_isahc_client::IsahcClient;
use oauth2_client::jwt_authorization_grant::Flow;
use oauth2_google::{GoogleProviderForServerToServerApps, GoogleScope};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let service_account_json_key_path = env::args()
        .nth(1)
        .ok_or("args service_account_json_key_path missing")?
        .parse::<PathBuf>()
        .map_err(|_| "args service_account_json_key_path invalid")?;

    run(service_account_json_key_path).await
}

async fn run(service_account_json_key_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let service_account_json_key_bytes = fs::read(service_account_json_key_path)?;

    let assertion = create_from_service_account_json_key(
        service_account_json_key_bytes,
        &[GoogleScope::AndroidPublisher.to_string()],
    )?;

    let flow = Flow::new(IsahcClient::new()?);
    let provider = GoogleProviderForServerToServerApps::new(assertion)?;

    let access_token_body = flow.execute(&provider, None).await?;

    println!("access_token_body: {access_token_body:?}");

    Ok(())
}
