use uuid::Uuid;
use actix::prelude::{Recipient,Message};


type Socket=Recipient<WsJob>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsJob>,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct WsJob(pub String);

#[derive(Message)]
#[rtype(result="()")]
pub struct WsJudgeResult(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}
