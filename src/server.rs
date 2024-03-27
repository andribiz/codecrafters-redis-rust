use anyhow::Result;
use bytes::BytesMut;
use std::io::Cursor;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::command::Command;
use crate::db::{ArcDB, DB};
use crate::resp::Resp;

pub struct Server {
    listener: TcpListener,
    db: ArcDB,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        Server {
            listener,
            db: Arc::new(DB::new()),
        }
    }

    pub async fn run(&self) {
        loop {
            match self.listener.accept().await {
                Ok((stream, _)) => {
                    println!("accepted new connection");
                    let mut handler = Handler {
                        stream,
                        db: self.db.clone(),
                    };
                    tokio::spawn(async move {
                        handler.run().await;
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}

struct Handler {
    stream: TcpStream,
    db: ArcDB,
}

impl Handler {
    pub async fn route(&self, buf: BytesMut) -> Result<Resp> {
        let mut cursor_buf = Cursor::new(&buf[..]);
        let resp_command = Resp::decode(&mut cursor_buf)?;
        let command = Command::from_resp(resp_command)?;
        let res = command.execute(&self.db).await;
        Ok(res)
    }

    pub async fn run(&mut self) {
        loop {
            let mut buf = BytesMut::with_capacity(1024);
            let len = self
                .stream
                .read_buf(&mut buf)
                .await
                .expect("error read stream");
            if len == 0 {
                break;
            }

            let res = match self.route(buf).await {
                Ok(res) => res,
                Err(e) => Resp::Error(e.to_string()),
            };
            let _ = self.stream.write_all(res.to_string().as_bytes()).await;
        }
    }
}
