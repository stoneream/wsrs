use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawRequest {
    pub operation: Operation,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    CreateRoom,
    JoinRoom,
    LeaveRoom,
    SendMessage,
}

impl RawRequest {
    pub fn new(operation: Operation, data: Option<serde_json::Value>) -> Self {
        RawRequest { operation, data }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn parse(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
