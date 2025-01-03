use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tracing::{error, info};
use tungstenite::Message;

type ClientId = uuid::Uuid;

struct ServerState {
    clients: HashMap<ClientId, SplitSink<WebSocketStream<TcpStream>, Message>>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    fn add_client(
        &mut self,
        id: ClientId,
        tx: futures::stream::SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
    ) {
        self.clients.insert(id, tx);
    }

    fn remove_client(&mut self, id: &ClientId) {
        self.clients.remove(id);
    }
}

async fn broadcast_message(state: &Arc<Mutex<ServerState>>, message: &Message, from: &ClientId) {
    // todo ブロードキャスト時にロックがかかるのでパフォーマンスに懸念がある
    // ロックをかけずに送信キューを作成して送信処理を非同期で行うべき
    let mut locked = state.lock().await;

    for (to, sink) in locked.clients.iter_mut() {
        // 自分自身には送信しない
        if to == from {
            continue;
        }

        if let Err(e) = sink.send(message.clone()).await {
            error!("Error sending message to client: {}", e);
        }
    }
}

async fn handle_connection(
    state: Arc<Mutex<ServerState>>,
    websocket: WebSocketStream<TcpStream>,
    socket_addr: SocketAddr,
    client_id: ClientId,
) {
    let (tx, mut rx) = websocket.split();

    // クライアントを登録
    {
        let mut locked = state.lock().await;
        locked.add_client(client_id, tx);
    }

    info!("Client {} connected from {}", client_id, socket_addr);

    while let Some(message) = rx.next().await {
        match message {
            Ok(Message::Text(text)) => {
                info!("Received message: {}", text);

                let message = Message::Text(text);
                broadcast_message(&state, &message, &client_id).await;
            }
            Ok(Message::Close(_)) => {
                info!("Client {} disconnected", client_id);
                break;
            }
            Err(e) => {
                error!("Error reading message: {}", e);
                break;
            }
            _ => continue,
        }
    }

    // クライアントを削除
    {
        let mut locked = state.lock().await;
        locked.remove_client(&client_id);
    }
    info!("Client {} disconnected", client_id);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let state = Arc::new(Mutex::new(ServerState::new()));

    let listener = {
        let addr = "127.0.0.1";
        let port = 9001;
        let listener = TcpListener::bind(format!("{}:{}", addr, port)).await?;

        info!("Server started at {}:{}", addr, port);

        listener
    };

    loop {
        let (stream, socket_addr) = listener.accept().await?;

        let state_clone = state.clone();
        let client_id = uuid::Uuid::new_v4();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await;

            match ws_stream {
                Ok(websocket) => {
                    handle_connection(state_clone, websocket, socket_addr, client_id).await;
                }
                Err(e) => {
                    error!("Failed to establish connection: {}", e);
                }
            }
        });
    }
}
