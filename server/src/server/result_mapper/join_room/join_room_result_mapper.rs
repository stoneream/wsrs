use crate::server::result_mapper::abstract_result_mapper::AbstractResultMapper;
use crate::usecase::join_room_usecase::join_room_handler::{
    JoinRoomHandlerError, JoinRoomHandlerOutput,
};
use shared_types::payload::join_room::join_room_response::{
    JoinRoomErrorResponse, JoinRoomSuccessResponse, JoinRoomSuccessResponseType,
};

pub struct JoinRoomResultMapper;

impl
    AbstractResultMapper<
        JoinRoomHandlerOutput,
        JoinRoomHandlerError,
        JoinRoomSuccessResponse,
        JoinRoomErrorResponse,
    > for JoinRoomResultMapper
{
    fn success(output: &JoinRoomHandlerOutput) -> JoinRoomSuccessResponse {
        JoinRoomSuccessResponse::new()
    }

    fn error(error: &JoinRoomHandlerError) -> JoinRoomErrorResponse {
        match error {
            JoinRoomHandlerError::AlreadyJoined => {
                JoinRoomErrorResponse::new(JoinRoomSuccessResponseType::AlreadyJoined)
            }
            JoinRoomHandlerError::RoomNotFound => {
                JoinRoomErrorResponse::new(JoinRoomSuccessResponseType::RoomNotFound)
            }
        }
    }
}
