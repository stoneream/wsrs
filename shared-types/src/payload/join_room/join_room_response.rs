use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JoinRoomSuccessResponse {}

impl JoinRoomSuccessResponse {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JoinRoomErrorResponse {
    pub error_type: JoinRoomErrorResponseType
}

impl JoinRoomErrorResponse {
    pub fn new(error_type: JoinRoomErrorResponseType) -> Self {
        Self { error_type }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JoinRoomErrorResponseType {
    RoomNotFound,
    AlreadyJoined,
}
