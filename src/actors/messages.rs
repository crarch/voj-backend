use uuid::Uuid;
use actix::prelude::{Recipient,Message};
use super::Job;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsJob>,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct WsJob(pub Job);

#[derive(Message)]
#[rtype(result="()")]
pub struct WsJudgeResult(pub Job);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}
