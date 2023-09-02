use std::net::SocketAddr;

use tokio::net::TcpStream;


async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await.expect("Error during websocket handshake");
    println!("Connection established: {}", addr);

    // let (outgoing, incoming) = ws_stream.split();
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let addr = "127.0.0.1:8080";
}
