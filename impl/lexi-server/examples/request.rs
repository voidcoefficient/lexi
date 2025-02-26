use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let server_address = dotenv::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:9991".to_string());
    let mut stream = TcpStream::connect(server_address).await?;

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
