use std::process::exit;

use anyhow::Result;
use lexi_server::server::Server;
use tracing::{Level, error, info};

fn tracing_setup() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .with_thread_ids(true)
        .init()
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_setup();
    info!("starting lexi server");

    let host = dotenv::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = dotenv::var("PORT").unwrap_or("9991".to_string());
    let server = Server::new(host, port);
    match server.serve().await {
        Ok(_) => {
            exit(0);
        }
        Err(err) => {
            error!(
                "could not bind to address {}. is this port being used?",
                &server.server_address()
            );
            error!("got error: {}", err);
            exit(1);
        }
    };
}
