// TCP stands for transmission control protocol.
use std::{thread, time};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();
    let mut socket = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);
    while let Ok((mut stream, peer)) = socket.accept().await {
        println!("Incoming connection from: {}", peer.to_string());
        tokio::spawn(async move {
            println!("thread starting {} starting", peer.to_string());
            let (reader, mut writer) = stream.split();
            let mut buf_reader = BufReader::new(reader);
            let mut buf = vec![];
            loop {
                match buf_reader.read_until(b'\n', &mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            println!("EOF received");
                            break;
                        }
                    }
                    Err(e) => println!("Error receiving message: {}", e),
                }
                let buf_string = String::from_utf8_lossy(&buf);
                let data: Vec<String> = buf_string
                    .split(";")
                    .map(|x| x.to_string().replace("\n", ""))
                    .collect();
                println!("Received message: {:?}", data);
                buf.clear();
            }
            println!("thread {} finishing", peer.to_string());
        });
    }
}
