use actix::Addr;
use actix_web::web::Data;
use actix::prelude::{Recipient};


type Socket=Recipient<WsMessage>;

mod judgerws;
mod messages;
mod judgers;

pub use judgers::*;
pub use judgerws::*;
pub use messages::*;

use crate::{MongoDB,Queue};
use crate::models::query_first_job;
use bson::Bson;

pub async fn call_back(
    srv: Data<Addr<Judgers>>
){
    srv.do_send(WsMessage("hihi".to_string()));
}



pub async fn push_job(
    judgers:Data<Addr<Judgers>>,
    mongo:MongoDB,
    queue:Queue
){
    while let Some(job)=query_first_job(mongo.clone(),queue.clone()).await{
        judgers.do_send(WsMessage(Bson::from(job).to_string()));
    }
}
