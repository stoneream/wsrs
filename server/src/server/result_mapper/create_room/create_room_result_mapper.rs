use shared_types::payload::create_room::create_room_response::{
    CreateRoomErrorResponse, CreateRoomSuccessResponse, CreateRoomSuccessResponseType,
};
use crate::server::result_mapper::abstract_result_mapper::AbstractResultMapper;
use crate::usecase::create_room_usecase::create_room_handler::{
    CreateRoomHandlerError, CreateRoomHandlerOutput,
};

pub struct CreateRoomResultMapper {}

impl
    AbstractResultMapper<
        CreateRoomHandlerOutput,
        CreateRoomHandlerError,
        CreateRoomSuccessResponse,
        CreateRoomErrorResponse,
    > for CreateRoomResultMapper
{
    fn success(output: &CreateRoomHandlerOutput) -> CreateRoomSuccessResponse {
        CreateRoomSuccessResponse::new(output.room_id.to_string())
    }

    fn error(error: &CreateRoomHandlerError) -> CreateRoomErrorResponse {
        match error {
            CreateRoomHandlerError::AlreadyJoined => {
                CreateRoomErrorResponse::new(CreateRoomSuccessResponseType::AlreadyJoined)
            }
        }
    }
}
