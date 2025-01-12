use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JoinRoomRequestData {
    pub room_id: String,
}

impl JoinRoomRequestData {
    pub fn new(room_id: String) -> Self {
        Self { room_id }
    }

    pub fn to_value(&self) -> Result<Value, Error> {
        serde_json::to_value(self)
    }
}
