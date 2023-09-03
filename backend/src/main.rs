use std::{env, io::Error};

use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};
use futures_util::{future, StreamExt, TryStreamExt};


async fn handle_connection(raw_stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await.expect("Error during websocket handshake");
    // println!("Connection established: {}", addr);

    let (outgoing, incoming) = ws_stream.split();
    incoming.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(outgoing)
        .await
        .expect("Failed to forward messages")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let addr = "127.0.0.1:8080";

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}
