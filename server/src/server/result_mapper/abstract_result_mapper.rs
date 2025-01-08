pub trait AbstractResultMapper<HandlerOutput, HandlerError, ResponseOutput, ResponseError> {
    fn success(output: &HandlerOutput) -> ResponseOutput;

    fn error(error: &HandlerError) -> ResponseError;
}
