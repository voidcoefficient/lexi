use std::{net::SocketAddr, process::exit};

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    spawn,
};
use tracing::{Level, debug, error, info, instrument};

fn tracing_setup() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .compact()
        .with_thread_ids(true)
        .init()
}

#[instrument]
async fn process(stream: TcpStream, address: SocketAddr) -> Result<()> {
    info!("processing request from {:?}", address);

    let mut buf = BufReader::new(stream);
    let mut request = String::new();
    match buf.read_line(&mut request).await {
        Ok(0) => {}
        Ok(_) => {
            debug!("{}", request);
            request.clear();
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_setup();
    info!("starting lexi server");

    let server_address = dotenv::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:9991".to_string());
    info!("listening on {}", &server_address);
    if let Ok(listener) = TcpListener::bind(&server_address).await {
        loop {
            let (socket, address) = listener.accept().await?;
            spawn(async move { process(socket, address).await });
        }
    }

    error!(
        "could not bind to address {}. is this port being used?",
        &server_address
    );
    exit(1);
}
