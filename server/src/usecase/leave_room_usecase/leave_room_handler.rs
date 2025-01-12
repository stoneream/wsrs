use crate::domain::user::User;
use crate::state::room_manager::RoomManager;
use crate::usecase::abstract_handler::AbstractHandler;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct LeaveRoomHandlerInput {
    pub user: Arc<User>,
}

impl LeaveRoomHandlerInput {
    pub fn new(user: Arc<User>) -> Self {
        Self { user }
    }
}

#[derive(Debug)]
pub struct LeaveRoomHandlerOutput {}

impl LeaveRoomHandlerOutput {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum LeaveRoomHandlerError {
    NotJoined,
}

pub struct LeaveRoomHandler {
    room_manager: Arc<Mutex<RoomManager>>,
}

#[async_trait]
impl AbstractHandler for LeaveRoomHandler {
    type Input = LeaveRoomHandlerInput;
    type Output = LeaveRoomHandlerOutput;
    type Error = LeaveRoomHandlerError;

    async fn run(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut room_manager = self.room_manager.lock().await;

        // 現在入室しているルームを取得
        let room_id_to_leave = {
            let rooms = room_manager.get_rooms();
            let maybe_room = rooms
                .iter()
                .find(|room| room.members.contains_key(&input.user.user_id));
            maybe_room.map(|r| r.room_id.clone())
        };

        let room = match room_id_to_leave {
            Some(room_id) => room_manager.get_room_mut(&room_id),
            None => None,
        };

        match room {
            Some(room) => {
                // ルームからユーザーを削除
                room.remove_member(input.user.user_id);
                Ok(LeaveRoomHandlerOutput {})
            }
            None => Err(LeaveRoomHandlerError::NotJoined),
        }
    }
}

impl LeaveRoomHandler {
    pub fn new(room_manager: Arc<Mutex<RoomManager>>) -> Self {
        Self { room_manager }
    }
}
