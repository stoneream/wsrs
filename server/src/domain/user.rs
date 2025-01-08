use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;
use tungstenite::Message;

pub type UserId = uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub user_id: UserId,
    pub addr: SocketAddr,
    pub tx: UnboundedSender<Message>,
}

impl User {
    pub fn new(addr: SocketAddr, tx: UnboundedSender<Message>) -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
            addr,
            tx,
        }
    }

    pub fn send(&self, message: Message) {
        let _ = self.tx.send(message);
    }
}
