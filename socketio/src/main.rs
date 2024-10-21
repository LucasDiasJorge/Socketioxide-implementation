use tokio::net::{TcpListener};
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:64000".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Can't bind");

    println!("WebSocket server is listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    write.send(Message::Text("Hello from the server! Connected".to_string()))
        .await
        .expect("Error sending message");

    while let Some(msg) = read.next().await {
        let msg = msg.expect("Error reading message");

        if msg.is_text() || msg.is_binary() {
            write.send(Message::Text("Hello from the server!".to_string()))
                .await
                .expect("Error sending message");
        }

        println!("Received a message: {:?}", msg.to_string());
    }
}
