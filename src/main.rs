use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::{error, info};
use tungstenite::{accept, Error as WsError, Message, Utf8Bytes, WebSocket};
use std::sync::mpsc;

type ClientId = uuid::Uuid;

struct ServerState {
    clients: HashMap<ClientId, Arc<Mutex<WebSocket<TcpStream>>>>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    fn add_client(&mut self, id: ClientId, ws: Arc<Mutex<WebSocket<TcpStream>>>) {
        self.clients.insert(id, ws);
    }

    fn remove_client(&mut self, id: &ClientId) {
        self.clients.remove(id);
    }

    fn broadcast(&self, message: &Utf8Bytes) {
        for (client_id, client_ws) in &self.clients {
            let mut ws = client_ws.lock().unwrap();
            if let Err(e) = ws.send(Message::Text(message.clone())) {
                error!("Failed to send message to {}: {}", client_id, e);
            }
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let addr = "127.0.0.1";
    let port = 9001;

    let server_state = Arc::new(Mutex::new(ServerState::new()));
    let (tx, rx) = mpsc::channel::<Utf8Bytes>();

    {
        let server_state = Arc::clone(&server_state);
        thread::spawn(move || {
            while let Ok(message) = rx.recv() {
                info!("Broadcasting message: {}", message);
                let state = server_state.lock().unwrap();
                state.broadcast(&message);
            }
        });
    }

    match TcpListener::bind(format!("{}:{}", addr, port)) {
        Ok(server) => {
            info!("Server started at {}:{}", addr, port);

            for stream in server.incoming() {
                let server_state = Arc::clone(&server_state);
                let tx = tx.clone();

                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            let client_id = uuid::Uuid::new_v4();

                            info!(
                                "New connection from {}, ID: {}",
                                stream.peer_addr().unwrap(),
                                client_id
                            );

                            let websocket = match accept(stream) {
                                Ok(ws) => Arc::new(Mutex::new(ws)),
                                Err(e) => {
                                    error!("Failed to establish connection: {}", e);
                                    return;
                                }
                            };

                            {
                                let mut server_state = server_state.lock().unwrap();
                                server_state.add_client(client_id, Arc::clone(&websocket));
                            }

                            loop {
                                let message = {
                                    let mut ws = websocket.lock().unwrap();
                                    ws.read()
                                };

                                match message {
                                    Ok(Message::Text(text)) => {
                                        if let Err(e) = tx.send(text) {
                                            error!(
                                                "Failed to send message to broadcast thread: {}",
                                                e
                                            );
                                        }
                                    }
                                    Ok(Message::Close(_)) | Err(WsError::ConnectionClosed) => {
                                        info!("Connection closed for client ID: {}", client_id);
                                        break;
                                    }
                                    Err(e) => {
                                        error!("Error reading message for client {}: {}", client_id, e);
                                        break;
                                    }
                                    _ => continue,
                                }
                            }

                            {
                                let mut server_state = server_state.lock().unwrap();
                                server_state.remove_client(&client_id);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);
                        continue;
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to bind to {}:{}. Error: {}", addr, port, e);
        }
    }
}
