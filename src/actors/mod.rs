use actix::{Actor};
use uuid::Uuid;
use std::collections::HashMap;
use actix::Addr;

use actix_web::web::Data;
use actix::prelude::{Context, Handler, Recipient};


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



