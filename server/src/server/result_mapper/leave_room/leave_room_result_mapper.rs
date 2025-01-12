use crate::server::result_mapper::abstract_result_mapper::AbstractResultMapper;
use crate::usecase::leave_room_usecase::leave_room_handler::{
    LeaveRoomHandlerError, LeaveRoomHandlerOutput,
};
use shared_types::payload::leave_room::leave_room_response::{
    LeaveRoomErrorResponse, LeaveRoomErrorResponseType, LeaveRoomSuccessResponse,
};

pub struct LeaveRoomResultMapper;

impl
    AbstractResultMapper<
        LeaveRoomHandlerOutput,
        LeaveRoomHandlerError,
        LeaveRoomSuccessResponse,
        LeaveRoomErrorResponse,
    > for LeaveRoomResultMapper
{
    fn success(_: &LeaveRoomHandlerOutput) -> LeaveRoomSuccessResponse {
        LeaveRoomSuccessResponse::new()
    }

    fn error(error: &LeaveRoomHandlerError) -> LeaveRoomErrorResponse {
        match error {
            LeaveRoomHandlerError::NotJoined => {
                LeaveRoomErrorResponse::new(LeaveRoomErrorResponseType::NotJoined)
            }
        }
    }
}
