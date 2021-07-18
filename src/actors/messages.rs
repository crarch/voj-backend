use uuid::Uuid;
use actix::prelude::{Recipient,Message};


type Socket=Recipient<WsMessage>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct WsMessage(pub String);

