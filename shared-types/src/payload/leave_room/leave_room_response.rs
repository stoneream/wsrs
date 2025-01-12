use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LeaveRoomSuccessResponse {
}

impl LeaveRoomSuccessResponse {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LeaveRoomErrorResponse {
    pub error_type: LeaveRoomErrorResponseType,
}

impl LeaveRoomErrorResponse {
    pub fn new(error_type: LeaveRoomErrorResponseType) -> Self {
        Self { error_type }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaveRoomErrorResponseType {
    NotJoined,
}
