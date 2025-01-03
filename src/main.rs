mod server;
mod state;
mod domain;
mod usecase;

use crate::state::room_manager::RoomManager;
use crate::state::user_manager::UserManager;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tracing::{error, info};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let listener = {
        let addr = "127.0.0.1";
        let port = 9001;
        let listener = TcpListener::bind(format!("{}:{}", addr, port)).await?;

        info!("Server started at {}:{}", addr, port);

        listener
    };

    // 状態管理用の構造体を初期化
    let room_manager = Arc::new(Mutex::new(RoomManager::new()));
    let user_manager = Arc::new(Mutex::new(UserManager::new()));

    loop {
        let (stream, socket_addr) = listener.accept().await?;

        let room_manager_clone = room_manager.clone();
        let user_manager_clone = user_manager.clone();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await;

            match ws_stream {
                Ok(ws_stream) => {
                    server::connection::handle_connection(
                        room_manager_clone,
                        user_manager_clone,
                        ws_stream,
                        socket_addr,
                    ).await;
                }
                Err(e) => {
                    error!("Failed to establish connection: {}", e);
                }
            }
        });
    }
}
