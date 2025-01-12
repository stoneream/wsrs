use crate::server::result_mapper::abstract_result_mapper::AbstractResultMapper;
use crate::usecase::send_message_usecase::send_message_handler::{
    SendMessageHandlerError, SendMessageHandlerOutput,
};
use shared_types::payload::send_message::send_message_response::{
    SendMessageErrorResponse, SendMessageErrorResponseType, SendMessageSuccessResponse,
};

pub struct SendMessageResultMapper;

impl
    AbstractResultMapper<
        SendMessageHandlerOutput,
        SendMessageHandlerError,
        SendMessageSuccessResponse,
        SendMessageErrorResponse,
    > for SendMessageResultMapper
{
    fn success(output: &SendMessageHandlerOutput) -> SendMessageSuccessResponse {
        SendMessageSuccessResponse::new(output.from_user_id.to_string(), output.message.clone())
    }

    fn error(error: &SendMessageHandlerError) -> SendMessageErrorResponse {
        match error {
            SendMessageHandlerError::NotJoined => {
                SendMessageErrorResponse::new(SendMessageErrorResponseType::NotJoined)
            }
        }
    }
}
