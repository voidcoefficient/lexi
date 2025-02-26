use std::net::SocketAddr;

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    spawn,
};
use tracing::{debug, info, instrument};

#[derive(Debug)]
pub struct Server {
    host: String,
    port: String,
}

impl Server {
    pub fn new<S: ToString>(host: S, port: S) -> Self {
        Self {
            host: host.to_string(),
            port: port.to_string(),
        }
    }

    #[instrument]
    pub async fn serve(&self) -> Result<()> {
        info!("listening on {}", &self.server_address());
        if let Ok(listener) = TcpListener::bind(&self.server_address()).await {
            loop {
                let (mut socket, address) = listener.accept().await?;
                spawn(async move { Self::process(&mut socket, address).await });
            }
        }

        Ok(())
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[instrument]
    async fn process(stream: &mut TcpStream, address: SocketAddr) -> Result<()> {
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

        buf.write_all(b"# lexi\r\nit's working!").await?;

        Ok(())
    }
}
