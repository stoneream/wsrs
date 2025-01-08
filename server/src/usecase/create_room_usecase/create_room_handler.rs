use crate::domain::room::{Room, RoomId};
use crate::domain::user::User;
use crate::state::room_manager::RoomManager;
use crate::usecase::abstract_handler::AbstractHandler;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CreateRoomHandlerInput {
    user: Arc<User>,
}

impl CreateRoomHandlerInput {
    pub fn new(user: Arc<User>) -> Self {
        Self { user }
    }
}

pub struct CreateRoomHandlerOutput {
    pub room_id: RoomId,
}

impl CreateRoomHandlerOutput {
    pub fn new(room_id: RoomId) -> Self {
        Self { room_id }
    }
}

pub enum CreateRoomHandlerError {
    AlreadyJoined,
}

pub struct CreateRoomHandler {
    room_manager: Arc<Mutex<RoomManager>>,
}

#[async_trait]
impl AbstractHandler for CreateRoomHandler {
    type Input = CreateRoomHandlerInput;
    type Output = CreateRoomHandlerOutput;
    type Error = CreateRoomHandlerError;

    async fn run(
        &self,
        input: CreateRoomHandlerInput,
    ) -> Result<CreateRoomHandlerOutput, CreateRoomHandlerError> {
        let mut room_manager = self.room_manager.lock().await;

        // すでに入室している場合はエラーを返す
        for room in room_manager.get_rooms() {
            if room.members.contains(&input.user.user_id) {
                return Err(CreateRoomHandlerError::AlreadyJoined);
            }
        }

        // ルームを作成・入室
        let mut room = Room::new();
        let room_id = room.room_id;
        room.add_member(input.user.user_id);
        room_manager.add_room(room);

        Ok(CreateRoomHandlerOutput::new(room_id))
    }
}

impl CreateRoomHandler {
    pub fn new(room_manager: Arc<Mutex<RoomManager>>) -> Self {
        Self { room_manager }
    }
}
