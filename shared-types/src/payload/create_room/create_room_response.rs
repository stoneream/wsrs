use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRoomSuccessResponse {
    pub room_id: String,
}

impl CreateRoomSuccessResponse {
    pub fn new(room_id: String) -> Self {
        Self {
            room_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRoomErrorResponse {
    pub error_type: CreateRoomSuccessResponseType,
}

impl CreateRoomErrorResponse {
    pub fn new(error_type: CreateRoomSuccessResponseType) -> Self {
        Self { error_type }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateRoomSuccessResponseType {
    AlreadyJoined,
}
