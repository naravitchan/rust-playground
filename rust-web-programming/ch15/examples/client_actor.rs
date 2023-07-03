use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("stream starting");
    stream.write_all(b"8.0;BYND;\n9.0;PLTR;\n10.0;PLTR").await?;
    println!("stream finished");
    Ok(())
}
