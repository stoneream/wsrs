use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageSuccessResponse {
    from_user_id: String,
    message: String,
}

impl SendMessageSuccessResponse {
    pub fn new(
        from_user_id: String,
        message: String,
    ) -> Self {
        Self { from_user_id, message }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageErrorResponse {
    pub error_type: SendMessageErrorResponseType
}

impl SendMessageErrorResponse {
    pub fn new(error_type: SendMessageErrorResponseType) -> Self {
        Self { error_type }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendMessageErrorResponseType {
    NotJoined,
}
