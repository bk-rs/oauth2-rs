use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    // TODO

    Ok(())
}
