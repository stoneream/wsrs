use async_trait::async_trait;

#[async_trait]
pub trait AbstractHandler {
    type Input;
    type Output;
    type Error;

    async fn run(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}
