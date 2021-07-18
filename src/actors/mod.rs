use actix::Addr;
use actix_web::web::Data;
use actix::prelude::{Recipient};


type Socket=Recipient<WsJob>;

mod judgerws;
mod messages;
mod judgers;
mod queue;

pub use judgers::*;
pub use judgerws::*;
pub use messages::*;
pub use queue::*;

use crate::{MongoDB};
use crate::models::query_first_job;
use bson::Bson;
use serde_json::Value;
pub async fn call_back(
    srv: Data<Addr<Judgers>>
){
    srv.do_send(WsJob("hihi".to_string()));
}



// pub async fn push_job(
//     judgers:Data<Addr<Judgers>>,
//     mongo:MongoDB,
//     queue:Queue
// ){
//     while let Some(job)=query_first_job(mongo.clone(),queue.clone()).await{
//         let job=Bson::from(job);
//         let job:Value=job.into();
//         let job=job.to_string();
//         judgers.do_send(WsJob(job));
//     }
// }
