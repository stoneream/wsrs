use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendMessageRequestData {
    pub message: String,
}

impl SendMessageRequestData {
    pub fn new(
        message: String,
    ) -> Self {
        Self { message }
    }

    pub fn to_value(&self) -> Result<Value, Error> {
        serde_json::to_value(self)
    }
}
