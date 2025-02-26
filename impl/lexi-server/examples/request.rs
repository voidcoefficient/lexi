use anyhow::Result;
use lexi_server::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let server_address = dotenv::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:9991".to_string());
    let client = Client::default();
    client.connect(server_address).await?;

    Ok(())
}
