use crate::domain::user::UserId;

pub type RoomId = uuid::Uuid;

#[derive(Debug)]
pub struct Room {
    pub room_id: RoomId,
    pub members: Vec<UserId>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            room_id: uuid::Uuid::new_v4(),
            members: Vec::new(),
        }
    }
}
