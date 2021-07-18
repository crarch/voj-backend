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





pub async fn call_back(
    srv: Data<Addr<Judgers>>
){
    srv.do_send(WsJob("hihi".to_string()));
}



pub async fn push_job(
    queue:Data<Addr<Queue>>,
    job:String
){
    queue.do_send(WsJob(job));
}
