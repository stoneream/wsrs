use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JoinRoomRequestData {
    pub room_id: String,
}

impl JoinRoomRequestData {
    pub fn new(room_id: String) -> Self {
        Self { room_id }
    }
}
