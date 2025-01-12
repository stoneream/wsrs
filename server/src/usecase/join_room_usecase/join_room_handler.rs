use crate::domain::room::RoomId;
use crate::domain::user::User;
use crate::state::room_manager::RoomManager;
use crate::usecase::abstract_handler::AbstractHandler;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct JoinRoomHandlerInput {
    pub room_id: RoomId,
    pub user: Arc<User>,
}

impl JoinRoomHandlerInput {
    pub fn new(room_id: RoomId, user: Arc<User>) -> Self {
        Self { room_id, user }
    }
}

#[derive(Debug)]
pub struct JoinRoomHandlerOutput {}

impl JoinRoomHandlerOutput {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum JoinRoomHandlerError {
    RoomNotFound,
    AlreadyJoined,
}

pub struct JoinRoomHandler {
    room_manager: Arc<Mutex<RoomManager>>,
}

#[async_trait]
impl AbstractHandler for JoinRoomHandler {
    type Input = JoinRoomHandlerInput;
    type Output = JoinRoomHandlerOutput;
    type Error = JoinRoomHandlerError;

    async fn run(
        &self,
        input: JoinRoomHandlerInput,
    ) -> Result<JoinRoomHandlerOutput, JoinRoomHandlerError> {
        let mut room_manager = self.room_manager.lock().await;

        // すでに入室している場合はエラーを返す
        for room in room_manager.get_rooms() {
            if room.members.contains_key(&input.user.user_id) {
                return Err(JoinRoomHandlerError::AlreadyJoined);
            }
        }

        // ルームが存在しない場合はエラーを返す
        let room = match room_manager.get_room_mut(&input.room_id) {
            Some(room) => room,
            None => return Err(JoinRoomHandlerError::RoomNotFound),
        };

        // ルームに入室
        room.add_member(input.user);

        Ok(JoinRoomHandlerOutput::new())
    }
}

impl JoinRoomHandler {
    pub fn new(room_manager: Arc<Mutex<RoomManager>>) -> Self {
        Self { room_manager }
    }
}
