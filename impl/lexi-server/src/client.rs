use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[derive(Default)]
pub struct Client {}

impl Client {
    pub async fn connect<S: ToString>(&self, address: S) -> Result<()> {
        let mut stream = TcpStream::connect(address.to_string()).await?;

        stream.write_all(b"/hello/world LEXI::1\r\n").await?;
        let mut buf = BufReader::new(stream);
        let mut response = String::new();
        loop {
            match buf.read_line(&mut response).await {
                Ok(0) => {
                    print!("{}", response);
                    break;
                }
                Ok(_) => {}
                Err(_) => todo!(),
            }
        }

        Ok(())
    }
}
