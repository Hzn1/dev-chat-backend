use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{CloseFrame, Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};

mod models;
mod storage;

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_failed_upgrade(|error| println!("Error upgrading websocket: {}", error))
        .on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(utf8_bytes) => {
                    println!("Text received: {}", utf8_bytes);
                    let result = socket
                        .send(Message::Text(
                            format!("Echo back text: {}", utf8_bytes,).into(),
                        ))
                        .await;
                    if let Err(error) = result {
                        println!("Error sending: {}", error);
                        send_close_message(socket, 1011, &format!("Error occured: {}", error))
                            .await;
                        break;
                    }
                }
                Message::Binary(bytes) => {
                    println!("Received bytes of lenght: {}", bytes.len());
                    let result = socket
                        .send(Message::Text(
                            format!("Received bytes of length: {}", bytes.len()).into(),
                        ))
                        .await;
                    if let Err(error) = result {
                        println!("Error sending: {}", error);
                        send_close_message(socket, 1011, &format!("Error occured: {}", error))
                            .await;
                        break;
                    }
                }
                _ => {}
            }
        } else {
            let error = msg.err().unwrap();
            println!("Error receiving message: {:?}", error);
            send_close_message(socket, 1011, &format!("Error occured: {}", error)).await;
            break;
        }
    }
}

async fn send_close_message(mut socket: WebSocket, code: u16, reason: &str) {
    _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: code,
            reason: reason.into(),
        })))
        .await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    /* let cors = CorsLayer::new()
    .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET])
    .allow_credentials(true)
    .allow_headers([UPGRADE]); */

    let app = Router::new().route("/ws", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
