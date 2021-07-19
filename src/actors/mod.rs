use actix::Addr;
use actix_web::web::Data;


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
    job:String
){
    queue.do_send(WsJob(job));
}
