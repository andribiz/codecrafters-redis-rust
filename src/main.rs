use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                println!("accepted new connection");
                loop {
                    let mut buf = BytesMut::with_capacity(1024);
                    let len = stream.read_buf(&mut buf).await.expect("Panic");
                    if len == 0 {
                        break;
                    }
                    let _ = stream.write_all(b"+PONG\r\n").await;
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
