use actix::Addr;
use actix_web::web::Data;
use actix::prelude::{Recipient};

mod judgerws;
mod messages;
mod judgers;
mod queue;

pub use judgers::*;
pub use judgerws::*;
pub use messages::*;
pub use queue::*;

pub async fn push_job(
    queue:Data<Addr<Queue>>,
    job:String
){
    queue.do_send(WsJob(job));
}
