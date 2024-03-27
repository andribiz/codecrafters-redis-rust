mod command;
mod db;
mod resp;
mod server;
use clap::Parser;
use db::DBMode;

use crate::server::Server;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Port of the application
    #[arg(short, long, default_value_t = 6379)]
    port: u16,
    #[arg(
        short,
        long,
        value_parser,
        number_of_values = 2,
        require_equals = false
    )]
    replicaof: Option<Vec<String>>,
}

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let args = Cli::parse();

    // Uncomment this block to pass the first stage
    let mode = if let Some(_) = args.replicaof {
        DBMode::Slave
    } else {
        DBMode::Master
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();
    let server = Server::new(listener, mode);
    server.run().await;
}
