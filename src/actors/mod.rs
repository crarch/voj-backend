use actix::Addr;
use actix_web::web::Data;
use serde::{Deserialize,Serialize};
use bson::oid::ObjectId;
use bson::Document;

mod judgerws;
mod messages;
mod scheduler;
mod queue;

pub use scheduler::*;
pub use judgerws::*;
pub use messages::*;
pub use queue::*;

pub async fn push_job(
    queue:Data<Addr<Queue>>,
    job:JudgeJob
){
    queue.do_send(WsJob(job));
}

#[derive(Debug,Serialize,Deserialize)]
pub struct JudgeJob{
    pub _id:ObjectId,
    pub success:bool,
    pub test_bench:Document,
    pub question_id:u32,
    pub user_id:u32,
    pub code:String,
    pub submit_time:u32,
}

