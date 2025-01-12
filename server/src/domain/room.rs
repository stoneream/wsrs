use crate::domain::user::{User, UserId};
use std::collections::HashMap;
use std::sync::Arc;

pub type RoomId = uuid::Uuid;

#[derive(Debug)]
pub struct Room {
    pub room_id: RoomId,
    pub members: HashMap<UserId, Arc<User>>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            room_id: uuid::Uuid::new_v4(),
            members: HashMap::new(),
        }
    }

    pub fn add_member(&mut self, user: Arc<User>) {
        self.members.insert(user.user_id, user);
    }

    pub fn remove_member(&mut self, user_id: UserId) {
        self.members.remove(&user_id);
    }
}
