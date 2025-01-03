use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Request {
    pub operation: Operation,
    pub data: Option<Data>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Operation {
    CreateRoom,
    JoinRoom,
    LeaveRoom,
    SendMessage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Data {
    CreateRoom {
        room_id: String
    },
    SendMessage {
        text: String
    }
}

impl Request {
    pub fn parse(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
