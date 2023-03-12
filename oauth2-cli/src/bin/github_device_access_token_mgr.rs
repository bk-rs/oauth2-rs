#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // TODO

    Ok(())
}
