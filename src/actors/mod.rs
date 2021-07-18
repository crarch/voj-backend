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





pub async fn call_back(
    srv: Data<Addr<Judgers>>
){
    srv.do_send(WsMessage("hihi".to_string()));
}



