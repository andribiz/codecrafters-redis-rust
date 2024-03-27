mod command;
mod resp;
use std::io::Cursor;

use crate::command::Command;
use crate::resp::Resp;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

fn route(command: Command) -> Resp {
    match command {
        Command::Ping => Resp::String(String::from("PONG")),
        Command::Echo(param) => Resp::String(param),
    }
}

async fn handler(stream: &mut TcpStream) {
    loop {
        let mut buf = BytesMut::with_capacity(1024);
        let len = stream.read_buf(&mut buf).await.expect("error read stream");
        if len == 0 {
            break;
        }

        let mut cursor_buf = Cursor::new(&buf[..]);
        let resp_command = match Resp::decode(&mut cursor_buf) {
            Ok(val) => val,
            Err(e) => {
                let resp = Resp::Error(String::from("invalid command")).to_string();
                let _ = stream.write_all(resp.as_bytes()).await;
                println!("error: {}", e);
                break;
            }
        };
        let commands = match Command::from_resp(resp_command) {
            Ok(val) => val,
            Err(_) => return,
        };

        let res = route(commands);
        let _ = stream.write_all(res.to_string().as_bytes()).await;
    }
}

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
                tokio::spawn(async move {
                    handler(&mut stream).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
