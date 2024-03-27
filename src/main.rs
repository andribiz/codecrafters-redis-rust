mod command;
mod db;
mod resp;
mod server;
use clap::Parser;

use crate::server::Server;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 6379)]
    port: usize,
}

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let args = Args::parse();

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();
    let server = Server::new(listener);
    server.run().await;
}
