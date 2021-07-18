use uuid::Uuid;
use actix::prelude::{Recipient,Message};
use bson::document::Document;


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

#[derive(Message)]
#[rtype(result="()")]
pub struct Job(pub Document);
