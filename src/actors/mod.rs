
use actix::{Actor};




use uuid::Uuid;
use std::collections::HashMap;
use actix::Addr;

use actix_web::web::Data;
use actix::prelude::{Context, Handler, Recipient};


type Socket=Recipient<WsMessage>;

mod judgerws;
mod messages;

pub use judgerws::*;
pub use messages::*;



pub struct Judgers{
    sessions:HashMap<Uuid,Socket>,
}

impl Default for Judgers{
    fn default()->Judgers{
        Judgers{
            sessions:HashMap::new(),
        }
    }
}


pub async fn call_back(
    srv: Data<Addr<Judgers>>
){
    srv.do_send(WsMessage("hihi".to_string()));
    // actix_rt::spawn(async move {
    //     let mut interval = time::interval(Duration::from_secs(2));
    //     loop {
    //         interval.tick().await;
    //         srv.do_send(WsMessage("hihi".to_string()));
    //     }
    // });
}


impl Actor for Judgers {
    type Context = Context<Self>;
}


impl Handler<WsMessage> for Judgers{
    type Result=();
    
    fn handle(&mut self,msg:WsMessage,_ctx:&mut Context<Self>)->Self::Result{
        let WsMessage(msg)=msg;
        self.send_message_to_all(&msg);
    }
}

impl Handler<Connect> for Judgers{
    type Result=();
    
    fn handle(&mut self,msg:Connect,_ctx:&mut Context<Self>)->Self::Result{
        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );
        
        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
    
}

impl Judgers {
    pub fn send_message_to_all(&self, message: &str) {
        self.sessions.iter().for_each(|(_,socket_recipient)| socket_recipient.do_send(WsMessage(message.to_owned())).unwrap());
    }
    
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }

}

