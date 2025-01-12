use crate::domain::user::User;
use crate::server::result_mapper::abstract_result_mapper::AbstractResultMapper;
use crate::server::result_mapper::create_room::create_room_result_mapper::CreateRoomResultMapper;
use crate::server::result_mapper::join_room::join_room_result_mapper::JoinRoomResultMapper;
use crate::server::result_mapper::leave_room::leave_room_result_mapper::LeaveRoomResultMapper;
use crate::server::result_mapper::send_message::send_message_result_mapper::SendMessageResultMapper;
use crate::state::room_manager::RoomManager;
use crate::state::user_manager::UserManager;
use crate::usecase::abstract_handler::AbstractHandler;
use crate::usecase::create_room_usecase::create_room_handler::{
    CreateRoomHandler, CreateRoomHandlerInput,
};
use crate::usecase::join_room_usecase::join_room_handler::{JoinRoomHandler, JoinRoomHandlerInput};
use crate::usecase::leave_room_usecase::leave_room_handler::{
    LeaveRoomHandler, LeaveRoomHandlerInput,
};
use crate::usecase::send_message_usecase::send_message_handler::{
    SendMessageHandler, SendMessageHandlerInput,
};
use futures::{SinkExt, StreamExt};
use shared_types::payload::join_room::join_room_request_data::JoinRoomRequestData;
use shared_types::payload::raw_request::{Operation, RawRequest};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use tracing::{error, info};
use tungstenite::Message;
use shared_types::payload::send_message::send_message_request_data::SendMessageRequestData;

pub async fn handle_connection(
    room_manager: Arc<Mutex<RoomManager>>,
    user_manager: Arc<Mutex<UserManager>>,
    ws_stream: WebSocketStream<TcpStream>,
    socket_addr: SocketAddr,
) {
    let (sink, stream) = ws_stream.split();

    // 送信チャンネルを作成、ユーザーと紐付けて管理
    let (tx, rx) = mpsc::unbounded_channel::<Message>();
    let user = Arc::new(User::new(socket_addr, tx));

    // ユーザーを管理下に追加
    {
        let mut locked = user_manager.lock().await;
        locked.add_user(user.clone());

        info!("User {} connected from {}", user.user_id, socket_addr);
    }

    let send_task = tokio::spawn(handle_outgoing(sink, rx));
    let receive_task = tokio::spawn(handle_incoming(
        room_manager.clone(),
        user_manager.clone(),
        user.clone(),
        stream,
    ));

    tokio::select! {
        _ = send_task => (),
        _ = receive_task => (),
    }

    // ユーザーを管理下から削除
    {
        let mut locked = user_manager.lock().await;
        locked.remove_user(&user.user_id);
        info!("User {} disconnected", user.user_id);
    }
}

// 送信タスク
async fn handle_outgoing(
    mut sink: futures::stream::SplitSink<WebSocketStream<TcpStream>, Message>,
    mut rx: mpsc::UnboundedReceiver<Message>,
) {
    while let Some(message) = rx.recv().await {
        if let Err(e) = sink.send(message).await {
            error!("Error sending message to client: {}", e);
        }
    }
}

// 受信タスク
async fn handle_incoming(
    room_manager: Arc<Mutex<RoomManager>>,
    user_manager: Arc<Mutex<UserManager>>,
    user: Arc<User>,
    mut stream: futures::stream::SplitStream<WebSocketStream<TcpStream>>,
) {
    // todo 長過ぎるメッセージを受信した場合に無視する
    while let Some(message) = stream.next().await {
        match message {
            Ok(message) => match message {
                Message::Text(text) => match RawRequest::parse(&text) {
                    Ok(raw_request) => {
                        route_operation(
                            room_manager.clone(),
                            user_manager.clone(),
                            user.clone(),
                            raw_request,
                        )
                        .await;
                    }
                    Err(e) => {
                        error!("Error parsing raw request: {}", e);
                    }
                },
                Message::Close(_) => {
                    info!("Connection closed");
                    break;
                }
                _ => continue,
            },
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        }
    }
}

async fn route_operation(
    room_manager: Arc<Mutex<RoomManager>>,
    user_manager: Arc<Mutex<UserManager>>,
    user: Arc<User>,
    raw_request: RawRequest,
) {
    match raw_request.operation {
        Operation::CreateRoom => {
            let handler = CreateRoomHandler::new(room_manager.clone());
            let input = CreateRoomHandlerInput::new(user.clone());
            let result = handler.run(input).await;

            match result {
                Ok(output) => {
                    let response = CreateRoomResultMapper::success(&output);
                    user.send(Message::text(serde_json::to_string(&response).unwrap()));
                }
                Err(error) => {
                    let response = CreateRoomResultMapper::error(&error);
                    user.send(Message::text(serde_json::to_string(&response).unwrap()));
                }
            }
        }
        Operation::JoinRoom => {
            if let Some(data) = raw_request.data {
                let data: JoinRoomRequestData = serde_json::from_value(data).unwrap();
                let handler = JoinRoomHandler::new(room_manager.clone());
                let input = JoinRoomHandlerInput::new(data.room_id.parse().unwrap(), user.clone());
                let result = handler.run(input).await;

                match result {
                    Ok(output) => {
                        let response = JoinRoomResultMapper::success(&output);
                        user.send(Message::text(serde_json::to_string(&response).unwrap()));
                    }
                    Err(error) => {
                        let response = JoinRoomResultMapper::error(&error);
                        user.send(Message::text(serde_json::to_string(&response).unwrap()));
                    }
                }
            }
        }
        Operation::LeaveRoom => {
            let handler = LeaveRoomHandler::new(room_manager.clone());
            let input = LeaveRoomHandlerInput::new(user.clone());
            let result = handler.run(input).await;

            match result {
                Ok(output) => {
                    let response = LeaveRoomResultMapper::success(&output);
                    user.send(Message::text(serde_json::to_string(&response).unwrap()));
                }
                Err(error) => {
                    let response = LeaveRoomResultMapper::error(&error);
                    user.send(Message::text(serde_json::to_string(&response).unwrap()));
                }
            }
        }
        Operation::SendMessage => {
            if let Some(data) = raw_request.data {
                let data: SendMessageRequestData = serde_json::from_value(data).unwrap();
                let handler = SendMessageHandler::new(room_manager.clone());
                let input = SendMessageHandlerInput::new(user.clone(), data.message);
                let result = handler.run(input).await;

                match result {
                    Ok(output) => {
                        let response = SendMessageResultMapper::success(&output);
                        for member in output.members {
                            member.send(Message::text(serde_json::to_string(&response).unwrap()));
                        }
                    }
                    Err(error) => {
                        let response = SendMessageResultMapper::error(&error);
                        user.send(Message::text(serde_json::to_string(&response).unwrap()));
                    }
                }
            }
        }
    }
}
