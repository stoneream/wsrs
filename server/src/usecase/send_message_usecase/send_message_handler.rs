use crate::domain::user::{User, UserId};
use crate::state::room_manager::RoomManager;
use crate::usecase::abstract_handler::AbstractHandler;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct SendMessageHandlerInput {
    pub user: Arc<User>,
    pub message: String,
}

impl SendMessageHandlerInput {
    pub fn new(user: Arc<User>, message: String) -> Self {
        Self { user, message }
    }
}

#[derive(Debug)]
pub struct SendMessageHandlerOutput {
    pub members: Vec<Arc<User>>,
    pub message: String,
    pub from_user_id: UserId,
}

impl SendMessageHandlerOutput {
    pub fn new(members: Vec<Arc<User>>, message: String, from_user_id: UserId) -> Self {
        Self {
            members,
            message,
            from_user_id,
        }
    }
}

pub enum SendMessageHandlerError {
    NotJoined,
}

pub struct SendMessageHandler {
    room_manager: Arc<Mutex<RoomManager>>,
}

impl SendMessageHandler {
    pub fn new(room_manager: Arc<Mutex<RoomManager>>) -> Self {
        Self { room_manager }
    }
}

#[async_trait]
impl AbstractHandler for SendMessageHandler {
    type Input = SendMessageHandlerInput;
    type Output = SendMessageHandlerOutput;
    type Error = SendMessageHandlerError;

    async fn run(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut room_manager = self.room_manager.lock().await;

        // 現在入室しているルームを取得
        let room_id_to_send = {
            let rooms = room_manager.get_rooms();
            let maybe_room = rooms
                .iter()
                .find(|room| room.members.contains_key(&input.user.user_id));
            maybe_room.map(|r| r.room_id.clone())
        };

        let room = match room_id_to_send {
            Some(room_id) => room_manager.get_room_mut(&room_id),
            None => None,
        };

        match room {
            Some(room) => {
                let members = room
                    .members
                    .clone()
                    .iter()
                    .map(|(_, user)| user.clone())
                    .collect();
                Ok(SendMessageHandlerOutput::new(
                    members,
                    input.message,
                    input.user.user_id,
                ))
            }
            None => Err(SendMessageHandlerError::NotJoined),
        }
    }
}
