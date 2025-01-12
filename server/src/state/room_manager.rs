use crate::domain::room::Room;
use crate::domain::room::RoomId;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RoomManager {
    rooms: HashMap<RoomId, Room>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.room_id, room);
    }

    pub fn remove_room(&mut self, room_id: &RoomId) {
        self.rooms.remove(room_id);
    }

    pub fn get_room(&self, room_id: &RoomId) -> Option<&Room> {
        self.rooms.get(room_id)
    }

    pub fn get_room_mut(&mut self, room_id: &RoomId) -> Option<&mut Room> {
        self.rooms.get_mut(room_id)
    }

    pub fn get_rooms(&self) -> Vec<&Room> {
        self.rooms.values().collect()
    }
}
